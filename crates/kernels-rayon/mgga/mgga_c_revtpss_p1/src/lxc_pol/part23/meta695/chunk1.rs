//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2443/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2443(t1412: f64, t9794: f64, t40609: f64, t4062: f64, t3994: f64, t40763: f64, t9793: f64, t2735: f64, t9792: f64, t1376: f64, t40769: f64, t10111: f64, t1386: f64, t9720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46825 = t9794 * t1412;
    let t46831 = 0.63807336860547134325e-3_f64 * t40609 * t4062;
    let t46833 = t9793 * t40763 * t3994;
    let t46835 = t2735 * t9792;
    let t46840 = 0.70398079132139197745e-2_f64 * t40769 * t1376;
    let t46856 = t10111 * t1386 * t9720;
    (t46825, t46831, t46833, t46835, t46840, t46856)
}
