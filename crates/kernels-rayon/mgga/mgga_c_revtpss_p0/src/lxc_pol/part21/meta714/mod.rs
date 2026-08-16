//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2548;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta714(t2661: f64, t3992: f64, t9810: f64, t9979: f64, t10111: f64, t1408: f64, t9720: f64, t1353: f64, t1414: f64, t685: f64, t9770: f64, t9775: f64, t46610: f64, t543: f64, t4003: f64, t9934: f64, t40735: f64, t535: f64, t235: f64, t5744: f64, t2453: f64, t9794: f64, t9935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46780, t46784, t46786, t46787, t46789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2548(t2661, t3992, t9810, t9979, t10111, t1408, t9720, t1353, t1414, t685, t9770, t9775);
        let (t46793, t46797, t46800, t46801, t46802, t46804) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2549(t2661, t3992, t46610, t543, t4003, t9934, t40735, t535, t235, t5744, t2453, t9794, t9935);
    (t46780, t46784, t46786, t46787, t46789, t46793, t46797, t46800, t46801, t46802, t46804)
}
