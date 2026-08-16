//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1725/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1725(t9769: f64, t9793: f64, t9794: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t4006: f64, t1399: f64, t2661: f64, t3992: f64, t9929: f64) -> (f64, f64, f64, f64) {
    let t46757 = t9793 * t9794 * t9769;
    let t46760 = 0.26776076960158126592e-7_f64 * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    let t46767 = t46766 * t4006;
    let t46771 = t2661 * t3992 * t9929 * t1399;
    (t46757, t46760, t46767, t46771)
}
