//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1151/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1151<F: Float>(t54279: F, t9427: F, t3237: F, t51371: F, t3242: F, t14011: F, t9634: F, t3232: F, t51372: F, t54265: F, t54268: F, t54269: F, t54272: F, t54273: F, t54276: F, t51388: F) -> (F, F) {
    let t54280 = t54279 * t9427;
    let t54283 = t51371 * t3237;
    let t54284 = 7.0 / 576.0 * t54283;
    let t54285 = t51371 * t3242;
    let t54286 = 7.0 / 144.0 * t54285;
    let t54287 = t14011 * t9634;
    let t54289 = t51371 * t3232;
    let t54290 = 7.0 / 144.0 * t54289;
    let t54291 = -t54265 / 96.0 + t54268 + t54269 / 48.0 - t54272 + t54273 / 192.0 + t54276 / 8.0 - t54280 / 64.0 - 7.0 / 144.0 * t51372 + t54284 - t54286 - t54287 / 768.0 - t54290;
    let t54293 = 119.0 / 1728.0 * t51388;
    (t54291, t54293)
}
