//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 580/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk580(t3115: f64, t443: f64, t453: f64, t36: f64, t3081: f64, t3082: f64, t3084: f64, t3086: f64, t3088: f64, t3095: f64, t3101: f64, t3106: f64, t3110: f64, t3113: f64) -> (f64, f64, f64, f64) {
    let t3116 = t443 * t3115;
    let t3117 = t453 * t3116;
    let t3118 = t36 * t3117;
    let t3120 = t3081 + 0.002518888888888889_f64 * t3082 - 0.0012594444444444445_f64 * t3084 + 0.003778333333333333_f64 * t3086 - 0.0018891666666666666_f64 * t3088 + 0.002099074074074074_f64 * t3095 - 0.007556666666666666_f64 * t3101 + 0.003778333333333333_f64 * t3106 + 0.011335_f64 * t3110 - 0.011335_f64 * t3113 + 0.0018891666666666666_f64 * t3118;
    (t3116, t3117, t3118, t3120)
}
