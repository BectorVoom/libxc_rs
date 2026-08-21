//! Explicit-SIMD transcendentals backed by `rmath`.
//!
//! Replaces transcendentals for `wide::f64x8` using `rmath` kernels.

use wide::f64x8;

/// `e^x` for `f64x8`, backed by `rmath::exp`.
#[inline(always)]
pub fn exp(x: f64x8) -> f64x8 {
    rmath::exp(x)
}

/// `ln(x)` for `f64x8`, backed by `rmath::ln`.
#[inline(always)]
pub fn ln(x: f64x8) -> f64x8 {
    rmath::ln(x)
}

/// Cube root for `f64x8`, backed by `rmath::cbrt`.
#[inline(always)]
pub fn cbrt(x: f64x8) -> f64x8 {
    rmath::cbrt(x)
}

/// x^(2/3) for `f64x8` = cbrt(x)^2
#[inline(always)]
pub fn pow_2_3(x: f64x8) -> f64x8 {
    let c = rmath::cbrt(x);
    c * c
}

/// x^(4/3) for `f64x8` = x * cbrt(x)
#[inline(always)]
pub fn pow_4_3(x: f64x8) -> f64x8 {
    x * rmath::cbrt(x)
}

/// x^(5/3) for `f64x8` = x * cbrt(x)^2
#[inline(always)]
pub fn pow_5_3(x: f64x8) -> f64x8 {
    let c = rmath::cbrt(x);
    x * c * c
}

/// x^(7/3) for `f64x8` = x * x * cbrt(x)
#[inline(always)]
pub fn pow_7_3(x: f64x8) -> f64x8 {
    x * x * rmath::cbrt(x)
}
