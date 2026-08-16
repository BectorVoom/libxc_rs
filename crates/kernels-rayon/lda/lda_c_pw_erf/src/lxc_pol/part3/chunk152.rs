//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 152/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk152(t391: f64, t40: f64, t357: f64, t85: f64, t1: f64, t60: f64, t119: f64, t155: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t392 = t40 * t391;
    let t393 = t357 * t85;
    let t394 = 0.019751789702565206_f64 * t393;
    let t395 = t60 * t1;
    let t397 = t119 * t155 * t84;
    (t392, t394, t395, t397)
}
