//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 646/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk646(t1035: f64, t1043: f64, t1041: f64, t632: f64, t1180: f64, t242: f64, t30: f64, t3667: f64, t633: f64, t409: f64, t621: f64, t138: f64, t634: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3868 = t1035 * t1043;
    let t3871 = 48.245938496077606_f64 * t1041 * t3868 * t632;
    let t3874 = 0.0034450798614814814_f64 * t30 * t1180 * t242;
    let t3875 = t3667 * t633;
    let t3877 = 6.0_f64 * t1041 * t3875;
    let t3878 = t409 * t621;
    let t3881 = 0.07123333333333333_f64 * t138 * t3878 * t634;
    (t3868, t3871, t3874, t3875, t3877, t3878, t3881)
}
