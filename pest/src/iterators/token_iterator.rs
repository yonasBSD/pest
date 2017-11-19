// pest. The Elegant Parser
// Copyright (C) 2017  Dragoș Tiselice
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use super::queueable_token::QueueableToken;
use super::super::inputs::{Input, position};
use super::super::RuleType;
use super::super::token::Token;

/// A `struct` containing `Token`s. It is returned by either
/// [`Pair::into_iter`](struct.Pair.html#method.into_iter) or
/// [`Pairs::into_iter`](struct.Pairs.html#method.into_iter)
#[derive(Clone, Debug)]
pub struct TokenIterator<'i, R, I: Input<'i>> {
    queue: Rc<Vec<QueueableToken<R>>>,
    input: Rc<I>,
    index: usize,
    start: usize,
    end: usize,
    __phantom: ::std::marker::PhantomData<&'i str>
}

pub fn new<'i, R: RuleType, I: Input<'i>>(
    queue: Rc<Vec<QueueableToken<R>>>,
    input: Rc<I>,
    start: usize,
    end: usize
) -> TokenIterator<'i, R, I> {
    TokenIterator {
        queue,
        input,
        index: 0,
        start,
        end,
        __phantom: ::std::marker::PhantomData
    }
}

impl<'i, R: RuleType, I: Input<'i>> Iterator for TokenIterator<'i, R, I> {
    type Item = Token<'i, R, I>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            return None;
        }

        let token = match self.queue[self.index] {
            QueueableToken::Start { pair, pos } => {
                let rule = match self.queue[pair] {
                    QueueableToken::End { rule, .. } => rule,
                    _ => unreachable!()
                };

                Token::Start {
                    rule,
                    // QueueableTokens are safely created.
                    pos: unsafe { position::new(self.input.clone(), pos) }
                }
            }
            QueueableToken::End { rule, pos } => {
                Token::End {
                    rule,
                    // QueueableTokens are safely created.
                    pos: unsafe { position::new(self.input.clone(), pos) }
                }
            }
        };

        self.index += 1;

        Some(token)
    }
}
