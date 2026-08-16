//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1071/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1071(t8278: f64, t8286: f64, t8291: f64, t11369: f64, t20033: f64, t85: f64, t15450: f64, t402: f64, t7376: f64, t75: f64, t8303: f64, t15453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20057 = 0.032530742648344574_f64 * t8278;
    let t20058 = 0.016265371324172287_f64 * t8286;
    let t20059 = 0.4815944609513912_f64 * t8291;
    let t20060 = 3076.1691063023386_f64 * t11369;
    let t20062 = 0.019751789702565206_f64 * t20033 * t85;
    let t20063 = 3.0_f64 * t15450;
    let t20066 = t7376 * t75 * t402;
    let t20067 = 0.5848223397455204_f64 * t20066;
    let t20068 = 103.89453539625518_f64 * t8303;
    let t20069 = 24.0_f64 * t15453;
    (t20057, t20058, t20059, t20060, t20062, t20063, t20067, t20068, t20069)
}
