//! `rmath::cbrt` against **glibc's** `cbrt`, bit for bit.
//!
//! # Why the reference is fetched with `dlopen`
//!
//! Every earlier comparison of `cbrt` against "libm" in this tree was made
//! inside a Rust binary, where the symbol `cbrt` is satisfied by
//! `compiler_builtins::math::libm_math::cbrt` -- a static definition the
//! linker prefers over `libm.so.6`'s. So a plain `extern "C" { fn cbrt(..) }`,
//! and `f64::cbrt`, both resolve to Rust's core-math port, not to glibc, and a
//! test written that way compares rmath against Rust and passes while the tree
//! disagrees with the glibc `cbrt` C libxc calls on more than half of inputs.
//! `dlsym` on a handle to `libm.so.6` returns glibc's own symbol, which static
//! resolution cannot substitute. That is the only reference this test trusts.
//!
//! Linux/glibc only, by construction.

#![cfg(all(target_os = "linux", target_env = "gnu"))]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::f64x8;
use std::ffi::{CString, c_char, c_int, c_void};

unsafe extern "C" {
    fn dlopen(file: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
}

type Cbrt = extern "C" fn(f64) -> f64;

/// glibc's `cbrt`, from `libm.so.6` itself.
fn glibc_cbrt() -> Cbrt {
    unsafe {
        let lib = CString::new("libm.so.6").unwrap();
        let h = dlopen(lib.as_ptr(), 2 /* RTLD_NOW */);
        assert!(!h.is_null(), "dlopen(libm.so.6) failed");
        let name = CString::new("cbrt").unwrap();
        let p = dlsym(h, name.as_ptr());
        assert!(!p.is_null(), "dlsym(cbrt) failed");
        std::mem::transmute::<*mut c_void, Cbrt>(p)
    }
}

/// Bit equality, with every NaN equal to every NaN (NaN payloads are
/// IEEE-unspecified and not what this test is about).
fn same(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

/// The inputs: specials, every power of two across the whole exponent range
/// with both neighbours and both signs, then random finite bit patterns --
/// which, unlike a log-uniform sweep, reach subnormals and every binade with
/// the frequency the representation itself has.
fn inputs() -> Vec<f64> {
    let mut v = vec![
        0.0, -0.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN,
        f64::MIN_POSITIVE, -f64::MIN_POSITIVE, f64::MAX, f64::MIN,
        f64::from_bits(1), f64::from_bits(0x000f_ffff_ffff_ffff),
        1.0, -1.0, 8.0, -8.0, 0.125, 27.0, 0.05, 1e-6, 3.7, 42.0,
    ];
    for k in -1074..=1023 {
        let p = 2f64.powi(k);
        for x in [p, f64::from_bits(p.to_bits() + 1), f64::from_bits(p.to_bits().saturating_sub(1))] {
            if x.is_finite() && x != 0.0 {
                v.push(x);
                v.push(-x);
            }
        }
    }
    let mut s = 0x243F_6A88_85A3_08D3u64;
    while v.len() < 4_000_000 {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let x = f64::from_bits(s);
        if x.is_finite() {
            v.push(x);
        }
    }
    v
}

#[test]
fn scalar_cbrt_is_bit_identical_to_glibc() {
    let g = glibc_cbrt();
    let (mut n, mut bad) = (0usize, Vec::new());
    for &x in &inputs() {
        n += 1;
        let (a, b) = (rmath::cbrt(x), g(x));
        if !same(a, b) && bad.len() < 10 {
            bad.push(format!("x={x:e} ({:#018x}): ours {:#018x} glibc {:#018x}", x.to_bits(), a.to_bits(), b.to_bits()));
        }
        assert!(same(rmath::cbrt_glibc(x), b), "cbrt_glibc({x:e}) != glibc");
    }
    assert!(bad.is_empty(), "rmath::cbrt differs from glibc on {} of {n}:\n{}", bad.len(), bad.join("\n"));
    println!("rmath::cbrt == glibc cbrt on all {n} inputs");
}

#[test]
fn simd_cbrt_is_bit_identical_to_glibc_in_every_lane() {
    let g = glibc_cbrt();
    let xs = inputs();
    let mut n = 0usize;
    for chunk in xs.chunks_exact(8) {
        let a: [f64; 8] = chunk.try_into().unwrap();
        let out = simd::cbrt(f64x8::new(a)).to_array();
        for lane in 0..8 {
            n += 1;
            assert!(
                same(out[lane], g(a[lane])),
                "simd::cbrt lane {lane}, x={:e}: ours {:#018x} glibc {:#018x}",
                a[lane], out[lane].to_bits(), g(a[lane]).to_bits()
            );
        }
    }
    println!("simd::cbrt == glibc cbrt on all {n} lanes");
}
