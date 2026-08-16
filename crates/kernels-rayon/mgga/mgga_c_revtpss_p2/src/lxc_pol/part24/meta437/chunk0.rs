//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1391/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1391(t10111: f64, t1386: f64, t9720: f64, t281: f64, t39644: f64, t40650: f64, t547: f64, t550: f64, t40688: f64, t2682: f64, t820: f64, t2735: f64, t5744: f64) -> (f64, f64, f64, f64, f64) {
    let t46856 = t10111 * t1386 * t9720;
    let t46885 = 0.47607864835161149081e-7_f64 * t39644 * t547 * t40650 * t550 * t281;
    let t46888 = t40688 * t547;
    let t46917 = t820 * t1386 * t2682;
    let t46929 = t2735 * t5744;
    (t46856, t46885, t46888, t46917, t46929)
}
