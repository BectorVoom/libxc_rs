//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1281/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1281<F: Float>(t11038: F, t21537: F, t2479: F, t2478: F, t2515: F, t4270: F, t11031: F, t7075: F, t2521: F, t4269: F, t7150: F, t7148: F, t3517: F, t9167: F, t21371: F, t21374: F, t4243: F) -> (F, F, F, F, F, F, F, F) {
    let t30015 = 0.62071215503128080361e4 * t21537 * t11038 * t2479;
    let t30018 = 2.0 * t2478 * t4270 * t2515;
    let t30021 = 0.96491876992155210402e2 * t7075 * t11031 * t2479;
    let t30024 = 0.16081979498692535067e2 * t2521 * t11031 * t2515;
    let t30025 = t4269 * t7150;
    let t30028 = 0.51726012919273400301e3 * t7148 * t30025 * t2479;
    let t30031 = 0.32163958997385070134e2 * t2521 * t3517 * t9167;
    let t30034 = 0.51726012919273400301e3 * t7148 * t11038 * t2515;
    let t30038 = 0.24955700379505800916e5 * t21371 * t4243 * t21374 * t2479;
    (t30015, t30018, t30021, t30024, t30028, t30031, t30034, t30038)
}
