//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1317/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1317(t51341: f64, t51358: f64, t54237: f64, t54239: f64, t54241: f64, t54246: f64, t54248: f64, t54251: f64, t54255: f64, t54258: f64, t54260: f64, t54261: f64) -> f64 {
    let t54263 = t54237 - t54239 - 7.0_f64 / 72.0_f64 * t51341 + t54241 / 48.0_f64 + t54246 / 24.0_f64 + t54248 / 192.0_f64 - 7.0_f64 / 288.0_f64 * t51358 - t54251 / 16.0_f64 - t54255 / 48.0_f64 + t54258 - t54260 - t54261 / 768.0_f64;
    t54263
}
