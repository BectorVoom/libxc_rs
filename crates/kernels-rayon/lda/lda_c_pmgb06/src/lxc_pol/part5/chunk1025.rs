//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1025/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1025(t1444: f64, t7674: f64, t16794: f64, t493: f64, t834: f64, t2010: f64, t2011: f64, t6123: f64, t6119: f64, t6286: f64, t432: f64, t7719: f64) -> (f64, f64, f64, f64, f64) {
    let t19265 = t1444 * t7674 / 15.0_f64;
    let t19268 = t493 * t16794 * t834 / 15.0_f64;
    let t19271 = 2.0_f64 / 15.0_f64 * t2010 * t6123 * t2011;
    let t19274 = 3.0_f64 / 5.0_f64 * t493 * t6119 * t6286;
    let t19276 = t432 * t7719 / 5.0_f64;
    (t19265, t19268, t19271, t19274, t19276)
}
