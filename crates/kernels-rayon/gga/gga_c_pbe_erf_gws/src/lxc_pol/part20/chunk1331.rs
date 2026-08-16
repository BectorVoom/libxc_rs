//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1331/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1331(t11754: f64, t4039: f64, t56998: f64, t57000: f64, t57002: f64, t57004: f64, t57006: f64, t57009: f64, t57011: f64, t57013: f64, t57015: f64, t57017: f64, t57019: f64, t57021: f64) -> f64 {
    let t57023 = t4039 * t11754;
    let t57025 = -t56998 / 24.0_f64 + 7.0_f64 / 144.0_f64 * t57000 - t57002 / 48.0_f64 + 5.0_f64 / 192.0_f64 * t57004 + t57006 / 384.0_f64 - t57009 / 96.0_f64 - t57011 / 32.0_f64 - t57013 / 48.0_f64 - 5.0_f64 / 96.0_f64 * t57015 + t57017 / 768.0_f64 + t57019 / 96.0_f64 + t57021 / 24.0_f64 + t57023 / 768.0_f64;
    t57025
}
