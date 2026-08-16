//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 296/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk296(t85: f64, t960: f64, t155: f64, t364: f64, t363: f64, t67: f64, t62: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t961 = t960 * t85;
    let t962 = 0.019751789702565206_f64 * t961;
    let t966 = t155 * t364;
    let t970 = t363 * t67;
    let t971 = 1.0_f64 / t970;
    let t972 = t62 * t971;
    let t973 = t370 * t370;
    (t962, t966, t970, t971, t972, t973)
}
