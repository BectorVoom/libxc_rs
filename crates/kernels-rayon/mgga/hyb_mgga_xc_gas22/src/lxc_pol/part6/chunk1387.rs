//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1387/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1387(t25520: f64, t25826: f64, t967: f64, t21393: f64, t21396: f64, t21427: f64, t21430: f64, t21433: f64, t21557: f64, t21560: f64, t25214: f64, t25217: f64, t25220: f64, t29819: f64) -> (f64, f64) {
    let t30127 = 0.2069040516770936012e4_f64 * t25826 * t25520 * t967;
    let t30137 = t21557 - 0.18602370370370370371e1_f64 * t21393 + 0.39862222222222222223e0_f64 * t21396 + t21560 + 0.27385555555555555556e0_f64 * t21430 - 0.1460562962962962963e1_f64 * t21427 + 0.27385555555555555556e0_f64 * t21433 - 0.1860237037037037037e1_f64 * t25214 + 0.15944888888888888889e1_f64 * t25217 - 0.59793333333333333334e0_f64 * t25220 + 0.1898925e1_f64 * t29819;
    (t30127, t30137)
}
