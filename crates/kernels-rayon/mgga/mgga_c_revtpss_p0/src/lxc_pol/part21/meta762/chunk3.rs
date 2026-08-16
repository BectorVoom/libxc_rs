//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2705/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2705(t2289: f64, t4288: f64, t13455: f64, t625: f64, t10209: f64, t1513: f64, t2366: f64, t28036: f64, t31035: f64, t46146: f64, t46148: f64, t46150: f64, t46152: f64, t46154: f64, t46157: f64, t49724: f64, t49760: f64, t49809: f64, t655: f64, t69: f64) -> f64 {
    let t49817 = t2289 * t4288;
    let t49818 = 11.0_f64 / 3.0_f64 * t49817;
    let t49819 = t625 * t13455;
    let t49828 = -11.0_f64 / 3.0_f64 * t46148 + t46154 / 3.0_f64 + t49724 - t69 * t655 * (t49760 + t49809) / 8.0_f64 + 22.0_f64 / 3.0_f64 * t46146 + 2.0_f64 * t46150 - 2.0_f64 * t46152 - t49818 + 6.0_f64 * t49819 + 3.0_f64 * t69 * t46157 * t1513 * t10209 - 9.0_f64 / 4.0_f64 * t31035 * t28036 * t2366;
    t49828
}
