//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 856/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk856<F: Float>(t11683: F, t6943: F, t11682: F, t3737: F, t6948: F, t6951: F, t640: F, t919: F, t3243: F, t128: F, t329: F, t2536: F, t3225: F, t773: F, t826: F, t10264: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11684 = t11683 * t6943;
    let t11685 = t11682 * t11684;
    let t11687 = t3737 * t6948;
    let t11688 = t11683 * t6951;
    let t11689 = t11687 * t11688;
    let t11691 = t640 * t919;
    let t11692 = t3243 * t11691;
    let t11694 = t128 * t329;
    let t11695 = t11694 * t2536;
    let t11696 = t3225 * t11695;
    let t11698 = t826 * t773;
    let t11699 = t10264 * t11698;
    (t11684, t11685, t11687, t11688, t11689, t11691, t11692, t11694, t11695, t11696, t11698, t11699)
}
