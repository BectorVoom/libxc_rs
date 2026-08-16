//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1308/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1308(t1410: f64, t2253: f64, t2256: f64, t3990: f64, t851: f64, t13925: f64, t13927: f64, t13930: f64, t13933: f64, t13937: f64, t13939: f64, t13941: f64, t13945: f64, t13952: f64, t13956: f64) -> f64 {
    let t15107 = t2253 * t1410;
    let t15108 = 2.0_f64 / 9.0_f64 * t15107;
    let t15109 = t2256 * t1410;
    let t15111 = t851 * t3990;
    let t15113 = -t13925 - t13927 - t15108 - 2.0_f64 / 9.0_f64 * t15109 + 8.0_f64 / 81.0_f64 * t15111 - t13930 + t13933 + t13937 + t13939 - t13941 - t13945 - t13952 + t13956;
    t15113
}
