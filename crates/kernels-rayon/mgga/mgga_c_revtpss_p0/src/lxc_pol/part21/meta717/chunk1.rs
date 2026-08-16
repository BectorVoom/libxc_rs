//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2555/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2555(t40688: f64, t547: f64, t46786: f64, t807: f64, t9400: f64, t9941: f64, t2689: f64, t9704: f64, t1386: f64, t2682: f64, t820: f64, t3940: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46888 = t40688 * t547;
    let t46889 = t46888 * t46786;
    let t46893 = t807 * t547 * t9941 * t9400;
    let t46895 = t2689 * t9704;
    let t46917 = t820 * t1386 * t2682;
    let t46918 = t46917 * t3940;
    (t46888, t46889, t46893, t46895, t46917, t46918)
}
