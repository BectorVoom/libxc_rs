//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1337/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1337(t54198: f64, t54183: f64, t54186: f64, t54188: f64, t54190: f64, t54192: f64, t54194: f64, t54196: f64, t54201: f64, t54203: f64, t54205: f64, t54207: f64, t54209: f64) -> f64 {
    let t55524 = 7.0_f64 / 288.0_f64 * t54198;
    let t55530 = t54183 / 48.0_f64 + t54186 / 24.0_f64 + t54188 / 12.0_f64 + t54190 / 48.0_f64 + t54192 / 64.0_f64 + t54194 / 64.0_f64 - t54196 / 16.0_f64 - t55524 + t54201 / 48.0_f64 - t54203 / 24.0_f64 - t54205 / 48.0_f64 - t54207 / 24.0_f64 + t54209 / 24.0_f64;
    t55530
}
