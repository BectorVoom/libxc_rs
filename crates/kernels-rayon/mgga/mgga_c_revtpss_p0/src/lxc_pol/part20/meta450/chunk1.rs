//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1716/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1716(t46590: f64, t808: f64, t9736: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64, t9775: f64, t9931: f64, t3989: f64, t9757: f64, t9761: f64, t9765: f64) -> (f64, f64, f64, f64, f64) {
    let t46592 = t9736 * t808 * t46590;
    let t46595 = t820 * t1408 * t9948;
    let t46596 = t46595 * t1416;
    let t46598 = t9775 * t9931;
    let t46600 = t3989 * t9757;
    let t46602 = t9765 * t9761;
    (t46592, t46596, t46598, t46600, t46602)
}
