//! Explicit-SIMD transcendentals backed by `rmath`.
//!
//! Replaces transcendentals for `wide::f64x8` using `rmath` bit-exact/correctly-rounded kernels.

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

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

/// `e^x - 1` for `f64x8`, backed by `rmath::expm1`.
#[inline(always)]
pub fn expm1(x: f64x8) -> f64x8 {
    rmath::expm1(x)
}

/// `ln(1 + x)` for `f64x8`, backed by `rmath::log1p`.
#[inline(always)]
pub fn log1p(x: f64x8) -> f64x8 {
    rmath::log1p(x)
}

/// `atan(x)` for `f64x8`, backed by `rmath::atan`.
#[inline(always)]
pub fn atan(x: f64x8) -> f64x8 {
    rmath::atan(x)
}

/// `atan2(y, x)` for `f64x8`, backed by `rmath::atan2`.
#[inline(always)]
pub fn atan2(y: f64x8, x: f64x8) -> f64x8 {
    rmath::atan2(y, x)
}

/// `tanh(x)` for `f64x8`, backed by `rmath::tanh`.
#[inline(always)]
pub fn tanh(x: f64x8) -> f64x8 {
    rmath::tanh(x)
}

/// `sinh(x)` for `f64x8`, backed by `rmath::sinh`.
#[inline(always)]
pub fn sinh(x: f64x8) -> f64x8 {
    rmath::sinh(x)
}

/// `cosh(x)` for `f64x8`, backed by `rmath::cosh`.
#[inline(always)]
pub fn cosh(x: f64x8) -> f64x8 {
    rmath::cosh(x)
}

/// `atanh(x)` for `f64x8`, backed by `rmath::atanh`.
#[inline(always)]
pub fn atanh(x: f64x8) -> f64x8 {
    rmath::atanh(x)
}

/// `sin(x)` for `f64x8`, backed by `rmath::sin`.
#[inline(always)]
pub fn sin(x: f64x8) -> f64x8 {
    rmath::sin(x)
}

/// `cos(x)` for `f64x8`, backed by `rmath::cos`.
#[inline(always)]
pub fn cos(x: f64x8) -> f64x8 {
    rmath::cos(x)
}

/// `tan(x)` for `f64x8`, backed by `rmath::tan`.
#[inline(always)]
pub fn tan(x: f64x8) -> f64x8 {
    rmath::tan(x)
}

/// `erf(x)` for `f64x8`, backed by `rmath::erf`.
#[inline(always)]
pub fn erf(x: f64x8) -> f64x8 {
    rmath::erf(x)
}

/// `erfc(x)` for `f64x8`, backed by `rmath::erfc`.
#[inline(always)]
pub fn erfc(x: f64x8) -> f64x8 {
    rmath::erfc(x)
}

/// `pow(x, y)` for `f64x8`, backed by `rmath::pow`.
#[inline(always)]
pub fn pow(x: f64x8, y: f64x8) -> f64x8 {
    rmath::pow(x, y)
}

/// `asin(x)` for `f64x8`, backed by `rmath::asin`.
#[inline(always)]
pub fn asin(x: f64x8) -> f64x8 {
    rmath::asin(x)
}

/// `acos(x)` for `f64x8`, backed by `rmath::acos`.
#[inline(always)]
pub fn acos(x: f64x8) -> f64x8 {
    rmath::acos(x)
}

#[inline(always)]
fn halley_step(w: f64x8, z: f64x8) -> f64x8 {
    let expmw = exp(-w);
    let residual = w - z * expmw;
    let denom = w + f64x8::splat(1.0) - (w + f64x8::splat(2.0)) / (f64x8::splat(2.0) * w + f64x8::splat(2.0)) * residual;
    let mask = (w + f64x8::splat(1.0)).abs().simd_lt(f64x8::splat(1.0e-300));
    let dw = mask.select(f64x8::splat(0.0), -residual / denom);
    w + dw
}

/// Principal branch of Lambert W function for `f64x8`.
#[inline(always)]
pub fn lambert_w(z: f64x8) -> f64x8 {
    let exp_1 = rmath::exp(1.0_f64);
    let inv_e = f64x8::splat(1.0_f64 / exp_1);
    let eps = f64x8::splat(1e-15_f64);
    let cbrt_eps = f64x8::splat(rmath::pow(1e-15_f64, 1.0_f64 / 3.0_f64));

    let small_res = z - z * z + f64x8::splat(1.5) * z * z * z;

    let branch_arg = (f64x8::splat(2.0_f64 * exp_1) * z + f64x8::splat(2.0_f64)).max(f64x8::splat(0.0));
    let branch_guess = branch_arg.sqrt() - f64x8::splat(1.0);
    let taylor_guess = small_res;
    let pos_z = z.max(f64x8::splat(1e-300));
    let lnz = ln(pos_z);
    let pos_lnz = lnz.max(f64x8::splat(1e-300));
    let asymp_guess = lnz - ln(pos_lnz);

    let is_near_branch = z.simd_le(f64x8::splat(-0.3140862435046707_f64));
    let is_taylor = z.simd_le(f64x8::splat(1.149876485041417_f64));

    let w0 = is_near_branch.select(branch_guess, is_taylor.select(taylor_guess, asymp_guess));

    let mut w = w0;
    for _ in 0..15 {
        w = halley_step(w, z);
    }

    let is_below_branch = (z + inv_e).simd_lt(f64x8::splat(-10.0_f64 * 1e-15_f64));
    let is_small_z = z.abs().simd_lt(cbrt_eps);

    is_below_branch.select(f64x8::splat(-1.0), is_small_z.select(small_res, w))
}

