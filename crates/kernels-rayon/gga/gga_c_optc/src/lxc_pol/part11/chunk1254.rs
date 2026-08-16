//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1254/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1254(t25174: f64, t55901: f64, t31479: f64, t322: f64, t40328: f64, t40356: f64, t49816: f64, t49822: f64, t49833: f64, t49860: f64, t49865: f64, t49869: f64, t50937: f64, t56700: f64, t56704: f64, t7449: f64, t862: f64) -> (f64, f64) {
    let t56708 = t25174 * t55901;
    let t56717 = -0.30524261601532767229e2_f64 * t7449 * t40356 * t50937 + t49816 / 54.0_f64 + 0.48838818562452427568e2_f64 * t49822 - t49833 / 27.0_f64 - t862 * t322 * t56700 / 12.0_f64 + t862 * t322 * t56704 / 72.0_f64 + t862 * t322 * t56708 / 6.0_f64 + 5.0_f64 / 972.0_f64 * t31479 - 0.12209704640613106892e2_f64 * t40328 + 7.0_f64 / 486.0_f64 * t49860 + t49865 / 216.0_f64 - 0.24419409281226213784e2_f64 * t49869;
    (t56708, t56717)
}
