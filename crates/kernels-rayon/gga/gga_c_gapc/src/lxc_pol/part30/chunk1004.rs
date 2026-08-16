//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1004/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1004(t12368: f64, t12389: f64, t12410: f64, t12431: f64, t576: f64, t3899: f64, t699: f64, t11614: f64, t11617: f64, t11621: f64, t11623: f64, t11627: f64, t11630: f64, t11634: f64, t11638: f64, t11641: f64, t11644: f64, t11649: f64, t11651: f64, t11653: f64) -> (f64, f64, f64, f64) {
    let t12433 = t12368 + t12389 + t12410 + t12431;
    let t12434 = t576 * t12433;
    let t12435 = t699 * t3899;
    let t12449 = -0.32829531147150437834e-4_f64 * t11614 - 0.32829531147150437834e-4_f64 * t11617 + 0.46971924784082831588e-5_f64 * t11621 - 0.32293198289056946717e-4_f64 * t11623 + 0.46971924784082831588e-4_f64 * t11627 + 0.29357452990051769742e-5_f64 * t11630 + 0.17399183805437348867e-6_f64 * t11634 + 0.29357452990051769742e-5_f64 * t11638 + 0.46971924784082831588e-4_f64 * t11641 - 0.68394856556563412154e-6_f64 * t11644 - 0.19948499828997661878e-6_f64 * t11649 + 0.61555370900907070939e-5_f64 * t11651 + 0.18788769913633132635e-3_f64 * t11653;
    (t12433, t12434, t12435, t12449)
}
