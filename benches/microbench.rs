#![feature(test)]
extern crate haphazard;
extern crate test;

use haphazard::*;
use std::sync::atomic::AtomicPtr;

use test::Bencher;

#[bench]
fn bench_acquire_and_release_hazptr(b: &mut Bencher) {
    b.iter(|| drop(HazPtrHolder::global()));
}

#[bench]
fn bench_protect_and_reset(b: &mut Bencher) {
    let x = AtomicPtr::new(Box::into_raw(Box::new(
        HazPtrObjectWrapper::with_global_domain(42),
    )));

    // As a reader:
    let mut h = HazPtrHolder::global();

    b.iter(|| {
        unsafe { h.protect(&x) }.expect("not null");
        h.reset();
    });
}
