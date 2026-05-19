//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1384/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1384<F: Float>(t4269: F, t7150: F, t2479: F, t7148: F, t2521: F, t3517: F, t9167: F, t11038: F, t2515: F, t21371: F, t21374: F, t4243: F) -> (F, F, F, F) {
    let t30025 = t4269 * t7150;
    let t30028 = F::cast_from(0.51726012919273400301e3_f64) * t7148 * t30025 * t2479;
    let t30031 = F::cast_from(0.32163958997385070134e2_f64) * t2521 * t3517 * t9167;
    let t30034 = F::cast_from(0.51726012919273400301e3_f64) * t7148 * t11038 * t2515;
    let t30038 = F::cast_from(0.24955700379505800916e5_f64) * t21371 * t4243 * t21374 * t2479;
    (t30028, t30031, t30034, t30038)
}
