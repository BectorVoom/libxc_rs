//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2538/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2538(t2782: f64, t4086: f64, t46394: f64, t543: f64, t3829: f64, t4010: f64, t808: f64, t9736: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64) -> (f64, f64, f64, f64) {
    let t46587 = t2782 * t4086 * t46394 * t543;
    let t46590 = t4010 * t3829;
    let t46592 = t9736 * t808 * t46590;
    let t46595 = t820 * t1408 * t9948;
    let t46596 = t46595 * t1416;
    (t46587, t46592, t46595, t46596)
}
