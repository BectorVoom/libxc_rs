//! Is `simd::pow` bit-identical to the scalar `rmath::pow` it replaces?
//!
//! `gga_x_pbepow:fxc:unpol` is the one triple out of 238 `pow`-using candidates
//! whose SIMD form produced a different output fingerprint, so the qualifier
//! rejected it (`reject-fingerprint`) despite a 1.93x speedup. That is the gate
//! working, but it leaves a question worth answering rather than assuming:
//! is the vector `pow` inexact somewhere, or is the difference confined to NaN
//! payloads -- which `AGENTS.md` already records as an artifact of deriving a
//! value down one expression path instead of two, and which carries no
//! information because IEEE 754 leaves NaN payloads unspecified?
//!
//!     cargo run --release -p libxc-rkernel-math --example pow_exact

use libxc_rkernel_math::{rmath, simd};
use libxc_rkernel_math::wide::f64x8;

fn lcg(s: &mut u64) -> f64 {
    *s = s
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    ((*s >> 11) as f64) / ((1u64 << 53) as f64)
}

fn main() {
    let mut s = 0x1357_9bdf_2468_ace0u64;
    let n = 200_000usize;

    let mut finite_diff = 0usize;
    let mut nan_diff = 0usize;
    let mut checked = 0usize;
    let mut worst_ulp = 0i64;
    let mut worst_at = (0.0f64, 0.0f64);

    // Bases spanning negatives through large positives, exponents covering the
    // fractional powers a maple2c body actually asks for.
    for _ in 0..n / 8 {
        let mut bx = [0.0f64; 8];
        let mut ex = [0.0f64; 8];
        for k in 0..8 {
            let u = lcg(&mut s);
            bx[k] = if u < 0.15 {
                // negative base: `pow` is NaN for a non-integer exponent
                -(10f64.powf(-6.0 + 8.0 * lcg(&mut s)))
            } else {
                10f64.powf(-8.0 + 12.0 * lcg(&mut s))
            };
            ex[k] = -4.0 + 8.0 * lcg(&mut s);
        }
        let v = simd::pow(f64x8::new(bx), f64x8::new(ex));
        let va: [f64; 8] = v.into();
        for k in 0..8 {
            let scalar = rmath::pow(bx[k], ex[k]);
            checked += 1;
            if va[k].to_bits() == scalar.to_bits() {
                continue;
            }
            if va[k].is_nan() && scalar.is_nan() {
                nan_diff += 1;
                continue;
            }
            finite_diff += 1;
            let u = (va[k].to_bits() as i64 - scalar.to_bits() as i64).abs();
            if u > worst_ulp {
                worst_ulp = u;
                worst_at = (bx[k], ex[k]);
            }
        }
    }

    println!("simd::pow vs rmath::pow, {checked} lanes");
    println!("  identical bits          : {}", checked - finite_diff - nan_diff);
    println!("  NaN vs NaN, payload only: {nan_diff}");
    println!("  FINITE differences      : {finite_diff}");
    if finite_diff > 0 {
        println!("  worst {worst_ulp} ulp at base={:e} exp={:e}", worst_at.0, worst_at.1);
    }
    println!();
    println!(
        "=> {}",
        if finite_diff == 0 {
            "vector pow is bit-exact on every finite result; any fingerprint \
             difference is NaN payload, which IEEE 754 leaves unspecified"
        } else {
            "vector pow differs on a FINITE result -- a real defect"
        }
    );
}
