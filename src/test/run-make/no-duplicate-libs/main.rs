// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "foo")] // linker should drop this library, no symbols used
#[link(name = "bar")] // symbol comes from this library
#[link(name = "foo")] // now linker picks up `foo` b/c `bar` library needs it
extern {
    fn bar();
}

fn main() {
    unsafe { bar() }
}
