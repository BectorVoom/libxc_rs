//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2437/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2437(t1419: f64, t9990: f64, t4089: f64, t40921: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64, t240: f64, t9991: f64, t3995: f64, t40488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46554 = t9990 * t1419;
    let t46570 = t40921 * t4089;
    let t46595 = t820 * t1408 * t9948;
    let t46596 = t46595 * t1416;
    let t46609 = t9991 * t240;
    let t46620 = t40488 * t3995;
    (t46554, t46570, t46595, t46596, t46609, t46620)
}
