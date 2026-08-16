//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1732/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1732(t10001: f64, t221: f64, t4019: f64, t9912: f64, t10111: f64, t1386: f64, t9720: f64, t1390: f64, t1399: f64, t685: f64, t9970: f64, t9976: f64) -> (f64, f64, f64) {
    let t46853 = t10001 * t4019 * t221 * t9912;
    let t46856 = t10111 * t1386 * t9720;
    let t46859 = t46856 * t1390 * t685 * t1399;
    let t46861 = t9976 * t9970;
    (t46853, t46859, t46861)
}
