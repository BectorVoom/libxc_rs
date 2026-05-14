//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 884/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk884<F: Float>(t12368: F, t12389: F, t12410: F, t12431: F, t576: F, t3899: F, t699: F, t11614: F, t11617: F, t11621: F, t11623: F, t11627: F, t11630: F, t11634: F, t11638: F, t11641: F, t11644: F, t11649: F, t11651: F, t11653: F) -> (F, F, F, F) {
    let t12433 = t12368 + t12389 + t12410 + t12431;
    let t12434 = t576 * t12433;
    let t12435 = t699 * t3899;
    let t12449 = -0.32829531147150437834e-4 * t11614 - 0.32829531147150437834e-4 * t11617 + 0.46971924784082831588e-5 * t11621 - 0.32293198289056946717e-4 * t11623 + 0.46971924784082831588e-4 * t11627 + 0.29357452990051769742e-5 * t11630 + 0.17399183805437348867e-6 * t11634 + 0.29357452990051769742e-5 * t11638 + 0.46971924784082831588e-4 * t11641 - 0.68394856556563412154e-6 * t11644 - 0.19948499828997661878e-6 * t11649 + 0.61555370900907070939e-5 * t11651 + 0.18788769913633132635e-3 * t11653;
    (t12433, t12434, t12435, t12449)
}
