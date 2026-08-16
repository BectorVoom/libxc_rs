//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1035/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1035(t3811: f64, t4488: f64, t4490: f64, t12118: f64, t4497: f64, t12100: f64, t12101: f64, t12102: f64, t12103: f64, t12104: f64, t12105: f64, t12108: f64, t12112: f64, t12117: f64, t12120: f64, t12125: f64) -> (f64, f64, f64) {
    let t12128 = 16.0_f64 / 15.0_f64 * t4488 * t4490 * t3811;
    let t12129 = t12118 * t4497;
    let t12130 = 32.0_f64 / 45.0_f64 * t12129;
    let t12131 = t12100 + t12101 - t12102 + t12103 + t12104 + t12105 + t12108 - t12112 - t12117 - t12120 - t12125 - t12128 + t12130;
    (t12128, t12130, t12131)
}
