//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1028/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1028(t3070: f64, t9270: f64, t1115: f64, t2397: f64, t2401: f64, t2408: f64, t2498: f64, t3207: f64, t335: f64, t4402: f64, t6175: f64, t6731: f64, t833: f64, t844: f64, t9215: f64, t9220: f64, t9224: f64, t9228: f64, t9232: f64, t9236: f64, t9241: f64, t9243: f64, t9249: f64, t9253: f64, t9255: f64, t9260: f64, t9265: f64) -> f64 {
    let t9272 = 7.0_f64 / 72.0_f64 * t9270 * t3070;
    let t9273 = 7.0_f64 / 288.0_f64 * t6175 - t3207 * t9215 / 16.0_f64 - t2408 * t9220 / 24.0_f64 - t335 * t9224 / 96.0_f64 + t2401 * t9228 / 16.0_f64 + t335 * t9232 / 48.0_f64 - t844 * t9236 / 48.0_f64 - t9241 * t9243 / 4.0_f64 - t9249 - t1115 * t4402 / 96.0_f64 + t9253 - t6731 - t335 * t9255 / 48.0_f64 - t844 * t9260 / 48.0_f64 + t9265 * t833 / 96.0_f64 + t2498 * t2397 / 48.0_f64 - t9272;
    t9273
}
