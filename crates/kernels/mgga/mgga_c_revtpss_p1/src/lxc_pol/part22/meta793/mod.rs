//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta793 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta793<F: Float>(t10103: F, t1432: F, t2470: F, t3999: F, t4066: F, t1438: F, t40317: F, t10065: F, t10069: F, t10084: F, t10079: F, t4089: F, t40921: F, t10073: F, t3829: F, t4010: F, t808: F, t9736: F, t1408: F, t820: F, t9948: F, t1416: F, t9775: F, t9931: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t46520, t46522, t46526, t46536, t46542, t46563, t46570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2887::<F>(t10103, t1432, t2470, t3999, t4066, t1438, t40317, t10065, t10069, t10084, t10079, t4089, t40921);
        let (t46572, t46592, t46595, t46596, t46598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2888::<F>(t10073, t10079, t3829, t4010, t808, t9736, t1408, t820, t9948, t1416, t9775, t9931);
    (t46520, t46522, t46526, t46536, t46542, t46563, t46570, t46572, t46592, t46595, t46596, t46598)
}
