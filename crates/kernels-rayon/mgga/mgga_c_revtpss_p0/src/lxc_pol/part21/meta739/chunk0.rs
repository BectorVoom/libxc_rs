//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2592/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2592(t46289: f64, t46291: f64, t1892: f64, t9646: f64, t9648: f64, t1904: f64, t47567: f64, t14110: f64, t47530: f64, t1427: f64, t1903: f64, t22: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47759 = 3.0_f64 * t46289;
    let t47760 = 192.0_f64 * t46291;
    let t47764 = t9646 * t1892 * t9648;
    let t47772 = t47567 * t1904;
    let t47777 = t47530 * t14110;
    let t47781 = t9647 * t1427 * t1903 * t22;
    (t47759, t47760, t47764, t47772, t47777, t47781)
}
