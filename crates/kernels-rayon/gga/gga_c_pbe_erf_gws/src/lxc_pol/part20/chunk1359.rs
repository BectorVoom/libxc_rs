//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1359/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1359(t1192: f64, t13911: f64, t14420: f64, t15139: f64, t22343: f64, t2376: f64, t2408: f64, t2409: f64, t29775: f64, t3207: f64, t3703: f64, t39689: f64, t4052: f64, t53075: f64, t53943: f64, t53948: f64, t53953: f64, t53959: f64, t57358: f64, t57361: f64, t57371: f64, t57373: f64, t57375: f64, t57379: f64, t8793: f64, t9807: f64) -> f64 {
    let t57381 = -t3207 * t2409 * t2376 * t4052 * t3703 / 16.0_f64 + t2408 * t2409 * t2376 * t1192 * t9807 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t57358 + t57361 / 768.0_f64 + t39689 * t13911 / 48.0_f64 - t53943 + t29775 * t14420 / 24.0_f64 + t22343 * t15139 / 96.0_f64 + t53948 + t8793 * t53075 / 24.0_f64 - 7.0_f64 / 2304.0_f64 * t57371 + 7.0_f64 / 288.0_f64 * t57373 + t57375 / 48.0_f64 + t53953 + 35.0_f64 / 108.0_f64 * t53959 + t57379 / 16.0_f64;
    t57381
}
