//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1113/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1113<F: Float>(t24447: F, t27822: F, t458: F, t27798: F, t96925: F, t11176: F, t1433: F, t27807: F, t92: F, t97168: F, t1154: F, t2476: F, t6119: F, t729: F, t1131: F, t2574: F, t27819: F) -> (F, F, F, F, F, F, F) {
    let t109356 = t24447 * t458 * t27822;
    let t109357 = t109356 / 4.0;
    let t109358 = t96925 * t27798;
    let t109359 = t109358 / 3.0;
    let t109361 = t1433 * t11176 * t27807;
    let t109363 = t97168 * t92;
    let t109367 = t109363 * t729 * t6119 * t1154 * t2476;
    let t109372 = t27819 * t2574 * t6119 * t1131 * t2476;
    (t109356, t109357, t109358, t109359, t109361, t109367, t109372)
}
