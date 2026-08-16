//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1319/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1319(t21296: f64, t367: f64, t899: f64, t9427: f64, t3237: f64, t51371: f64, t3242: f64, t14011: f64, t9634: f64, t3232: f64, t51372: f64, t54265: f64, t54268: f64, t54269: f64, t54272: f64, t54273: f64, t54276: f64) -> f64 {
    let t54279 = t899 * t21296 * t367;
    let t54280 = t54279 * t9427;
    let t54283 = t51371 * t3237;
    let t54284 = 7.0_f64 / 576.0_f64 * t54283;
    let t54285 = t51371 * t3242;
    let t54286 = 7.0_f64 / 144.0_f64 * t54285;
    let t54287 = t14011 * t9634;
    let t54289 = t51371 * t3232;
    let t54290 = 7.0_f64 / 144.0_f64 * t54289;
    let t54291 = -t54265 / 96.0_f64 + t54268 + t54269 / 48.0_f64 - t54272 + t54273 / 192.0_f64 + t54276 / 8.0_f64 - t54280 / 64.0_f64 - 7.0_f64 / 144.0_f64 * t51372 + t54284 - t54286 - t54287 / 768.0_f64 - t54290;
    t54291
}
