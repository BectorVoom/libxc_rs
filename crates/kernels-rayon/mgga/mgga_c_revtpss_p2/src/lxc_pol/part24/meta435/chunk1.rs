//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1388/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1388(t1386: f64, t2482: f64, t2668: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t10111: f64, t1408: f64, t9720: f64, t40735: f64, t535: f64) -> (f64, f64, f64, f64, f64) {
    let t46740 = t2482 * t1386 * t2668;
    let t46760 = 0.26776076960158126592e-7_f64 * t40757 * t1376;
    let t46766 = t820 * t4000 * t2681;
    let t46784 = t10111 * t1408 * t9720;
    let t46800 = 455.0_f64 / 243.0_f64 * t40735 * t535;
    (t46740, t46760, t46766, t46784, t46800)
}
