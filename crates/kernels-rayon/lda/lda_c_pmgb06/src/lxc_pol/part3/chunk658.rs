//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 658/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk658(t123: f64, t290: f64, t317: f64, t4001: f64, t342: f64, t384: f64, t374: f64, t1227: f64, t73: f64, t1234: f64, t113: f64, t2778: f64, t301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4005 = 0.9247854820715865_f64 * t123 * t4001 * t290 * t317;
    let t4006 = t384 * t342;
    let t4013 = t384 * t374;
    let t4017 = t73 * t1227;
    let t4021 = t73 * t1234;
    let t4027 = 0.006715335817467199_f64 * t2778 * t113 * t301;
    (t4005, t4006, t4013, t4017, t4021, t4027)
}
