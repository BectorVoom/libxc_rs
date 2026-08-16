//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1032/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1032(t1115: f64, t2384: f64, t2397: f64, t2408: f64, t2503: f64, t3040: f64, t3066: f64, t3079: f64, t3207: f64, t4419: f64, t6746: f64, t6748: f64, t6805: f64, t844: f64, t9275: f64, t9285: f64, t9289: f64, t9290: f64, t9293: f64, t9299: f64, t9302: f64, t9307: f64, t9313: f64, t9317: f64) -> f64 {
    let t9320 = 35.0_f64 / 432.0_f64 * t9275 + t3040 * t2397 / 48.0_f64 - 7.0_f64 / 288.0_f64 * t6746 + t2384 * t2503 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t6748 - t2408 * t9285 / 12.0_f64 + t9289 - 35.0_f64 / 216.0_f64 * t9290 - t3207 * t9293 / 8.0_f64 - t3066 * t9299 / 16.0_f64 + t9302 * t3079 / 96.0_f64 + 7.0_f64 / 72.0_f64 * t6805 - t844 * t9307 / 48.0_f64 + t1115 * t4419 / 96.0_f64 - t844 * t9313 / 24.0_f64 - t844 * t9317 / 24.0_f64;
    t9320
}
