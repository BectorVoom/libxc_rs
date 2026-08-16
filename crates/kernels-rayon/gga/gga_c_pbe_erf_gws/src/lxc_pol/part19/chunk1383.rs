//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1383/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1383(t56998: f64, t57000: f64, t57002: f64, t57004: f64, t57006: f64, t57009: f64, t57011: f64, t57013: f64, t57015: f64, t57017: f64, t57019: f64, t57021: f64, t57023: f64) -> f64 {
    let t58670 = -t56998 / 12.0_f64 + 7.0_f64 / 72.0_f64 * t57000 - t57002 / 24.0_f64 + 5.0_f64 / 96.0_f64 * t57004 + t57006 / 192.0_f64 - t57009 / 48.0_f64 - t57011 / 16.0_f64 - t57013 / 24.0_f64 - 5.0_f64 / 48.0_f64 * t57015 + t57017 / 384.0_f64 + t57019 / 48.0_f64 + t57021 / 12.0_f64 + t57023 / 384.0_f64;
    t58670
}
