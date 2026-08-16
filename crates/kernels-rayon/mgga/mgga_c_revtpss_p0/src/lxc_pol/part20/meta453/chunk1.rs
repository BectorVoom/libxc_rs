//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1730/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1730(t1379: f64, t40846: f64, t550: f64, t816: f64, t1412: f64, t9794: f64, t1353: f64, t1399: f64, t9793: f64, t40609: f64, t4062: f64, t3994: f64, t40763: f64) -> (f64, f64, f64, f64, f64) {
    let t46824 = 0.12516778469694349359e-1_f64 * t1379 * t40846 * t550 * t816;
    let t46825 = t9794 * t1412;
    let t46826 = t1399 * t1353;
    let t46828 = t9793 * t46825 * t46826;
    let t46831 = 0.63807336860547134325e-3_f64 * t40609 * t4062;
    let t46833 = t9793 * t40763 * t3994;
    (t46824, t46826, t46828, t46831, t46833)
}
