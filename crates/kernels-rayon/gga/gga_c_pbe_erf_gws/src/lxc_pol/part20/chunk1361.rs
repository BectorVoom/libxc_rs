//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1361/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1361(t51870: f64, t51877: f64, t53784: f64, t53971: f64, t53976: f64, t53977: f64, t53980: f64, t53986: f64, t54430: f64, t55751: f64, t57386: f64, t57390: f64, t57393: f64, t57395: f64, t57398: f64, t8793: f64) -> f64 {
    let t57401 = t57386 / 192.0_f64 - t8793 * t53784 / 8.0_f64 - t53971 + t53976 - t57390 / 16.0_f64 - 35.0_f64 / 216.0_f64 * t53977 + t57393 / 24.0_f64 + t53980 + t53986 + t57395 / 48.0_f64 - t55751 + t54430 - t57398 / 48.0_f64 - t51870 + 35.0_f64 / 432.0_f64 * t51877;
    t57401
}
