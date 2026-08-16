//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2445/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2445(t46786: f64, t46888: f64, t1386: f64, t2682: f64, t820: f64, t2735: f64, t5744: f64, t4086: f64, t9801: f64, t9846: f64, t1320: f64, t9545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46889 = t46888 * t46786;
    let t46917 = t820 * t1386 * t2682;
    let t46929 = t2735 * t5744;
    let t46946 = t9801 * t4086;
    let t46947 = t46946 * t9846;
    let t46963 = 16.0_f64 * t1320 * t9545;
    (t46889, t46917, t46929, t46946, t46947, t46963)
}
