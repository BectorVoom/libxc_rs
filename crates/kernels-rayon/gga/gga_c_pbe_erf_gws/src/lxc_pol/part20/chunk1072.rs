//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1072/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1072(t12005: f64, t12009: f64, t12015: f64, t12021: f64, t12025: f64, t12031: f64, t12034: f64, t12038: f64, t12040: f64, t12047: f64, t12050: f64, t2253: f64, t2277: f64, t6579: f64, t9645: f64, t9658: f64) -> f64 {
    let t12053 = -t9645 - t12005 - t2253 * t12009 / 384.0_f64 - t2253 * t12015 / 768.0_f64 - t2253 * t12021 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t6579 * t12025 - 119.0_f64 / 1728.0_f64 * t9658 - t12031 + t12034 + t12038 - t12040 + t12047 - t2277 * t12050 / 1536.0_f64;
    t12053
}
