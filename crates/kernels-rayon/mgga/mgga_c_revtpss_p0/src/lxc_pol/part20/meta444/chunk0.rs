//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1699/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1699(t30: f64, t3889: f64, t3853: f64, t3860: f64, t10179: f64, t4147: f64, t513: f64, t9603: f64, t3834: f64, t2257: f64, t1344: f64, t3874: f64, t39456: f64, t9344: f64, t9605: f64, t9608: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t46298 = t3889 * t3889;
    let t46302 = t3860 * t3853;
    let t46303 = 72.0_f64 * t46302;
    let t46304 = t10179 * t4147;
    let t46310 = 1.0_f64 / t513 / t9603 / t30;
    let t46311 = t3834 * t3834;
    let t46317 = t2257 * t2257;
    let t46325 = piecewise3(t31, 0.0_f64, -56.0_f64 / 81.0_f64 * t46310 * t46311 + 16.0_f64 / 9.0_f64 * t9605 * t3834 * t2257 - 2.0_f64 / 3.0_f64 * t3874 * t46317 - 8.0_f64 / 9.0_f64 * t9608 * t9344 + 2.0_f64 / 3.0_f64 * t1344 * t39456);
    (t46298, t46303, t46304, t46311, t46317, t46325)
}
