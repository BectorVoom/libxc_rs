//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1122/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1122(t1972: f64, t6518: f64, t6783: f64, t2002: f64, t6499: f64, t153: f64, t1864: f64, t439: f64, t6123: f64, t16118: f64, t1859: f64, t16866: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20478 = 2.0_f64 / 15.0_f64 * t1972 * t6518;
    let t20480 = t1972 * t6783 / 15.0_f64;
    let t20482 = 2.0_f64 / 9.0_f64 * t2002 * t6499;
    let t20486 = 2.0_f64 / 15.0_f64 * t439 * t6123 * t153 * t1864;
    let t20490 = t439 * t16118 * t153 * t1859 / 9.0_f64;
    let t20491 = 2.0_f64 / 135.0_f64 * t16866;
    (t20478, t20480, t20482, t20486, t20490, t20491)
}
