//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1391/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1391(t54352: f64, t54356: f64, t54381: f64, t55608: f64, t55620: f64, t57195: f64, t57197: f64, t57199: f64, t57201: f64, t57204: f64, t57206: f64, t57208: f64, t57210: f64) -> f64 {
    let t58765 = -119.0_f64 / 432.0_f64 * t54352 + t55608 - 35.0_f64 / 54.0_f64 * t54356 + t55620 - t57195 / 192.0_f64 - t57197 / 96.0_f64 - t57199 / 96.0_f64 - 35.0_f64 / 108.0_f64 * t54381 + 7.0_f64 / 144.0_f64 * t57201 + t57204 / 12.0_f64 - 7.0_f64 / 144.0_f64 * t57206 + t57208 / 12.0_f64 + t57210 / 8.0_f64;
    t58765
}
