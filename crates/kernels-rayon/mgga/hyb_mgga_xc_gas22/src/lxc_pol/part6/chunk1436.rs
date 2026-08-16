//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1436/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1436(t4851: f64, t9488: f64, t9625: f64, t9778: f64, t11589: f64, t11594: f64, t22600: f64, t26103: f64, t26122: f64, t31126: f64, t4502: f64, t4505: f64, t4513: f64, t7602: f64, t9444: f64, t9458: f64, t9474: f64, t9598: f64, t9612: f64, t9624: f64, t9737: f64, t9742: f64) -> f64 {
    let t31179 = t9488 * t4851;
    let t31197 = t9625 * t9778;
    let t31205 = -11200.0_f64 / 9.0_f64 * t11589 * t9458 - 3200.0_f64 / 27.0_f64 * t9598 * t31179 + 3200.0_f64 / 27.0_f64 * t11594 * t9458 - 48.0_f64 * t26103 * t31126 - 720.0_f64 * t26122 * t9625 * t9444 - 64.0_f64 / 81.0_f64 * t22600 * t4502 - 3200.0_f64 / 3.0_f64 * t9612 * t31179 + 88.0_f64 / 27.0_f64 * t7602 * t4513 - 32.0_f64 / 27.0_f64 * t7602 * t4505 + 12.0_f64 * t9742 * t31197 - 180.0_f64 * t9624 * t9625 * t9474 + 252.0_f64 * t9737 * t31197;
    t31205
}
