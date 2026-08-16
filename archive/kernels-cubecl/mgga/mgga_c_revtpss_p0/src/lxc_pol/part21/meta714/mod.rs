//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta714 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2548;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta714<F: Float>(t2661: F, t3992: F, t9810: F, t9979: F, t10111: F, t1408: F, t9720: F, t1353: F, t1414: F, t685: F, t9770: F, t9775: F, t46610: F, t543: F, t4003: F, t9934: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t9794: F, t9935: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46780, t46784, t46786, t46787, t46789) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2548::<F>(t2661, t3992, t9810, t9979, t10111, t1408, t9720, t1353, t1414, t685, t9770, t9775);
        let (t46793, t46797, t46800, t46801, t46802, t46804) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2549::<F>(t2661, t3992, t46610, t543, t4003, t9934, t40735, t535, t235, t5744, t2453, t9794, t9935);
    (t46780, t46784, t46786, t46787, t46789, t46793, t46797, t46800, t46801, t46802, t46804)
}
