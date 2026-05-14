//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 462/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk462<F: Float>(t2712: F, t617: F, t2404: F, t332: F, t298: F, t181: F, t2394: F, t282: F, t129: F, t2254: F, t314: F, t604: F, t892: F, t1636: F, t291: F, t906: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2713 = t617 * t2712;
    let t2716 = t332 * t2404;
    let t2717 = t298 * t2716;
    let t2718 = t181 * t2717;
    let t2721 = t2394 * t282;
    let t2722 = t2721 * t129;
    let t2723 = t314 * t2254;
    let t2724 = t298 * t2723;
    let t2725 = t181 * t2724;
    let t2728 = t604 * t892;
    let t2732 = t1636 * t291 * t906;
    (t2713, t2716, t2718, t2721, t2722, t2723, t2725, t2728, t2732)
}
