//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta793(t10103: f64, t1432: f64, t2470: f64, t3999: f64, t4066: f64, t1438: f64, t40317: f64, t10065: f64, t10069: f64, t10084: f64, t10079: f64, t4089: f64, t40921: f64, t10073: f64, t3829: f64, t4010: f64, t808: f64, t9736: f64, t1408: f64, t820: f64, t9948: f64, t1416: f64, t9775: f64, t9931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46520, t46522, t46526, t46536, t46542, t46563, t46570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887(t10103, t1432, t2470, t3999, t4066, t1438, t40317, t10065, t10069, t10084, t10079, t4089, t40921);
        let (t46572, t46592, t46595, t46596, t46598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2888(t10073, t10079, t3829, t4010, t808, t9736, t1408, t820, t9948, t1416, t9775, t9931);
    (t46520, t46522, t46526, t46536, t46542, t46563, t46570, t46572, t46592, t46595, t46596, t46598)
}
