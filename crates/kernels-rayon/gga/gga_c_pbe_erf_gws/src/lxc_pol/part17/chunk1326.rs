//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1326/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1326(t51431: f64, t54338: f64, t54342: f64, t54345: f64, t54346: f64, t54348: f64, t54350: f64, t54352: f64, t54355: f64, t54356: f64, t54360: f64, t54362: f64) -> f64 {
    let t54364 = -5.0_f64 / 96.0_f64 * t54338 + t54342 / 48.0_f64 - t54345 - 5.0_f64 / 64.0_f64 * t54346 - t54348 / 48.0_f64 - t54350 / 96.0_f64 - 119.0_f64 / 1728.0_f64 * t54352 + t54355 - 35.0_f64 / 216.0_f64 * t54356 + 7.0_f64 / 144.0_f64 * t51431 + t54360 / 8.0_f64 + t54362 / 384.0_f64;
    t54364
}
