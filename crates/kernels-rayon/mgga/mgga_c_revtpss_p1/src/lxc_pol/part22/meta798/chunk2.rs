//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2898/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2898(t1413: f64, t46826: f64, t46835: f64, t1376: f64, t40769: f64, t10111: f64, t1386: f64, t9720: f64, t1390: f64, t1399: f64, t685: f64, t9970: f64, t9976: f64) -> (f64, f64, f64, f64, f64) {
    let t46837 = t46835 * t1413 * t46826;
    let t46840 = 0.70398079132139197745e-2_f64 * t40769 * t1376;
    let t46856 = t10111 * t1386 * t9720;
    let t46859 = t46856 * t1390 * t685 * t1399;
    let t46861 = t9976 * t9970;
    (t46837, t46840, t46856, t46859, t46861)
}
