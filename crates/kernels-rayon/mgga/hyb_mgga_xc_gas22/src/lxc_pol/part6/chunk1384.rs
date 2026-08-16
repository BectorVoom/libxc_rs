//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1384/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1384(t4269: f64, t7150: f64, t2479: f64, t7148: f64, t2521: f64, t3517: f64, t9167: f64, t11038: f64, t2515: f64, t21371: f64, t21374: f64, t4243: f64) -> (f64, f64, f64, f64) {
    let t30025 = t4269 * t7150;
    let t30028 = 0.51726012919273400301e3_f64 * t7148 * t30025 * t2479;
    let t30031 = 0.32163958997385070134e2_f64 * t2521 * t3517 * t9167;
    let t30034 = 0.51726012919273400301e3_f64 * t7148 * t11038 * t2515;
    let t30038 = 0.24955700379505800916e5_f64 * t21371 * t4243 * t21374 * t2479;
    (t30028, t30031, t30034, t30038)
}
