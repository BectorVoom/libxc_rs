//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 677/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk677(t6407: f64, t6427: f64, t1788: f64, t27: f64, t13: f64, t1758: f64, t533: f64, t1792: f64, t3649: f64, t3696: f64, t6364: f64, t6367: f64, t6370: f64, t6375: f64, t6377: f64, t6379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6428 = t6407 * t6427;
    let t6432 = 1.0_f64 / t1788 / t27;
    let t6433 = t13 * t6432;
    let t6434 = t1758 * t533;
    let t6435 = t6434 * t1792;
    let t6437 = 0.96490945932906628932e2_f64 * t6433 * t6435;
    let t6446 = -0.25319e1_f64 * t6364 + 0.16879333333333333333e1_f64 * t6367 - 0.19692555555555555555e1_f64 * t6370 - 0.93011851851851851854e0_f64 * t3649 + 0.13651666666666666667e0_f64 * t6375 - 0.27303333333333333333e0_f64 * t6377 - 0.3185388888888888889e0_f64 * t6379 - 0.36514074074074074075e0_f64 * t3696;
    (t6428, t6432, t6433, t6434, t6435, t6437, t6446)
}
