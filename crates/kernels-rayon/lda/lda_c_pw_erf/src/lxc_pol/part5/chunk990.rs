//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 990/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk990(t1: f64, t3921: f64, t5470: f64, t2260: f64, t3936: f64, t1410: f64, t2253: f64, t2256: f64, t3990: f64, t851: f64, t256: f64, t3932: f64, t850: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15015 = t5470 * t1 * t3921;
    let t15060 = t2260 * t3936;
    let t15107 = t2253 * t1410;
    let t15108 = 2.0_f64 / 9.0_f64 * t15107;
    let t15109 = t2256 * t1410;
    let t15111 = t851 * t3990;
    let t15123 = t850 * t3932 * t256;
    (t15015, t15060, t15108, t15109, t15111, t15123)
}
