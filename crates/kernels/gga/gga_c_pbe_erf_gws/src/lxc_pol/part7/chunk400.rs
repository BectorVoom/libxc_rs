//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 400/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk400<F: Float>(t1680: F, t199: F, t582: F, t662: F, t211: F, t174: F, t205: F, t332: F, t395: F, t628: F, t56: F, t641: F) -> (F, F, F, F, F, F, F) {
    let t1682 = 4.0 / 15.0 * t1680 * t199;
    let t1683 = t582 * t662;
    let t1684 = t211 * t1683;
    let t1685 = 8.0 / 45.0 * t1684;
    let t1687 = t174 * t332 * t205;
    let t1688 = 0.47988888888888888889e-1 * t1687;
    let t1689 = t395 * t628;
    let t1691 = t56 * t641;
    (t1682, t1683, t1685, t1687, t1688, t1689, t1691)
}
