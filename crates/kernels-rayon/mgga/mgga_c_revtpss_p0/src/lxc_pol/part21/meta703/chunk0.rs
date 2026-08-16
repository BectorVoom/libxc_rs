//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2526/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2526(t39454: f64, t521: f64, t1333: f64, t9413: f64, t30: f64, t513: f64, t9603: f64, t33: f64, t516: f64, t9615: f64, t10008: f64, t213: f64) -> (f64, f64, f64, f64, f64) {
    let t46291 = t39454 * t521;
    let t46297 = 480.0_f64 * t9413 * t1333;
    let t46310 = 1.0_f64 / t513 / t9603 / t30;
    let t46328 = 1.0_f64 / t516 / t9615 / t33;
    let t46350 = t213 * t10008;
    (t46291, t46297, t46310, t46328, t46350)
}
