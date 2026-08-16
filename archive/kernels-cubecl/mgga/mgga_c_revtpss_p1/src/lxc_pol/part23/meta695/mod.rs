//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta695 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2442;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta695<F: Float>(t2452: F, t40633: F, t46808: F, t547: F, t268: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F, t1379: F, t40846: F, t816: F, t1412: F, t9794: F, t40609: F, t4062: F, t3994: F, t40763: F, t9793: F, t2735: F, t9792: F, t1376: F, t40769: F, t10111: F, t1386: F, t9720: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46810, t46817, t46820, t46824) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2442::<F>(t2452, t40633, t46808, t547, t268, t40634, t550, t9718, t247, t548, t9722, t1379, t40846, t816);
        let (t46825, t46831, t46833, t46835, t46840, t46856) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2443::<F>(t1412, t9794, t40609, t4062, t3994, t40763, t9793, t2735, t9792, t1376, t40769, t10111, t1386, t9720);
    (t46810, t46817, t46820, t46824, t46825, t46831, t46833, t46835, t46840, t46856)
}
