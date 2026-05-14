//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 947/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk947<F: Float>(t29756: F, t83: F, t29758: F, t1339: F, t4495: F, t452: F, t1332: F, t488: F, t3238: F, t6538: F, t6557: F, t942: F, t29600: F, t4436: F, t1871: F, t4462: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29810 = t83 * t29756;
    let t29813 = t83 * t29758;
    let t29817 = t452 * t1339 * t4495;
    let t29822 = t1332 * t4495;
    let t29824 = t452 * t488 * t29822;
    let t29828 = t452 * t3238 * t6538;
    let t29831 = t6557 * t942;
    let t29833 = t452 * t488 * t29831;
    let t29836 = t83 * t29600;
    let t29839 = t1332 * t4436;
    let t29841 = t1871 * t488 * t29839;
    let t29845 = t447 * t1339 * t4462;
    (t29810, t29813, t29817, t29822, t29824, t29828, t29831, t29833, t29836, t29839, t29841, t29845)
}
