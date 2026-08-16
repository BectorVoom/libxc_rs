//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2877/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2877(t2240: f64, t2246: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t11149: f64, t78: f64, t12267: f64, t81: f64, t46: f64, t47: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45958 = t2240 * t2246;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46001 = 1.0_f64 / t78 / t11149;
    let t46014 = 1.0_f64 / t81 / t12267;
    let t46063 = t46 * t46;
    let t46065 = 1.0_f64 / t47 / t46063;
    (t45958, t45963, t45972, t46001, t46014, t46065)
}
