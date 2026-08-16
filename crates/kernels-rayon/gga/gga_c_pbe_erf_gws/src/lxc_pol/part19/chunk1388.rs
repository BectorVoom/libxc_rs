//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1388/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1388(t54293: f64, t54294: f64, t54305: f64, t55580: f64, t57127: f64, t57130: f64, t57132: f64, t57134: f64, t57138: f64, t57140: f64, t57142: f64, t57144: f64, t57146: f64) -> f64 {
    let t58730 = -t57127 / 2.0_f64 + t57130 / 4.0_f64 + t57132 / 24.0_f64 - t57134 / 192.0_f64 - t54293 - t54294 + t57138 / 12.0_f64 + t55580 - 119.0_f64 / 864.0_f64 * t54305 - t57140 / 384.0_f64 - 7.0_f64 / 72.0_f64 * t57142 - 7.0_f64 / 24.0_f64 * t57144 + 7.0_f64 / 72.0_f64 * t57146;
    t58730
}
