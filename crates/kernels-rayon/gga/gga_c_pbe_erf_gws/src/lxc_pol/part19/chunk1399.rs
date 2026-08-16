//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1399/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1399(t3200: f64, t335: f64, t338: f64, t4228: f64, t52582: f64, t52586: f64, t52589: f64, t54641: f64, t55090: f64, t55182: f64, t55904: f64, t55918: f64, t55936: f64, t55942: f64, t57668: f64, t57671: f64, t57674: f64, t57678: f64, t57685: f64, t8793: f64) -> f64 {
    let t58919 = -t57668 / 12.0_f64 + t57671 / 24.0_f64 - 35.0_f64 / 216.0_f64 * t55904 - t52582 - t55918 - t55936 - t57674 / 4.0_f64 - t55942 - t57678 / 192.0_f64 + 35.0_f64 / 108.0_f64 * t54641 - t8793 * t55090 / 12.0_f64 - t8793 * t55182 / 8.0_f64 - 35.0_f64 / 432.0_f64 * t52586 + t52589 - t335 * t338 * t3200 * t4228 / 48.0_f64 - t57685 / 2.0_f64;
    t58919
}
