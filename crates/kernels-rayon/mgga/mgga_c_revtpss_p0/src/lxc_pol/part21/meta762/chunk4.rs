//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2706/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2706(t114: f64, t10208: f64, t10254: f64, t13458: f64, t13509: f64, t2339: f64, t2340: f64, t2366: f64, t4263: f64, t4287: f64, t46143: f64, t46144: f64, t49698: f64, t49701: f64, t49702: f64, t49704: f64, t49828: f64, t665: f64, t69: f64) -> f64 {
    let t115 = 1.0_f64 < t114;
    let t49830 = piecewise3(t115, 0.0_f64, 154.0_f64 / 27.0_f64 * t49698 + t49701 - 4.0_f64 * t49702 - 2.0_f64 * t49704 - 9.0_f64 / 4.0_f64 * t69 * t10208 * t4287 * t2340 + 3.0_f64 / 4.0_f64 * t69 * t2339 * t13509 * t665 + 3.0_f64 / 4.0_f64 * t69 * t13458 * t2366 + t69 * t4263 * t10254 / 4.0_f64 + t46143 + 154.0_f64 / 9.0_f64 * t46144 + t49828);
    t49830
}
