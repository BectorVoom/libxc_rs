//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 684/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk684<F: Float>(t6407: F, t6427: F, t1788: F, t27: F, t13: F, t1758: F, t533: F, t1792: F, t3649: F, t3696: F, t6364: F, t6367: F, t6370: F, t6375: F, t6377: F, t6379: F) -> (F, F, F, F, F, F, F) {
    let t6428 = t6407 * t6427;
    let t6432 = F::cast_from(1.0_f64) / t1788 / t27;
    let t6433 = t13 * t6432;
    let t6434 = t1758 * t533;
    let t6435 = t6434 * t1792;
    let t6437 = F::cast_from(0.96490945932906628932e2_f64) * t6433 * t6435;
    let t6446 = -F::cast_from(0.25319e1_f64) * t6364 + F::cast_from(0.16879333333333333333e1_f64) * t6367 - F::cast_from(0.19692555555555555555e1_f64) * t6370 - F::cast_from(0.93011851851851851854e0_f64) * t3649 + F::cast_from(0.13651666666666666667e0_f64) * t6375 - F::cast_from(0.27303333333333333333e0_f64) * t6377 - F::cast_from(0.3185388888888888889e0_f64) * t6379 - F::cast_from(0.36514074074074074075e0_f64) * t3696;
    (t6428, t6432, t6433, t6434, t6435, t6437, t6446)
}
