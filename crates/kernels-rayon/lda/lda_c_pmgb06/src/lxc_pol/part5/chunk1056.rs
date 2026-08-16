//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1056/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1056(t131: f64, t178: f64, t19654: f64, t44: f64, t513: f64, t7628: f64, t6688: f64, t844: f64, t1837: f64, t2563: f64, t1972: f64, t6744: f64) -> (f64, f64, f64, f64, f64) {
    let t19658 = t19654 * t44 * t131 * t178 / 30.0_f64;
    let t19660 = t7628 * t513 / 30.0_f64;
    let t19662 = t6688 * t844 / 10.0_f64;
    let t19664 = t2563 * t1837 / 10.0_f64;
    let t19666 = 2.0_f64 / 15.0_f64 * t1972 * t6744;
    (t19658, t19660, t19662, t19664, t19666)
}
