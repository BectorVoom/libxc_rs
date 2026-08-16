//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2550;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta715<F: Float>(t1389: F, t268: F, t2452: F, t40633: F, t547: F, t9793: F, t9794: F, t9930: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F, t1379: F, t40846: F, t816: F, t1412: F, t1353: F, t1399: F, t40609: F, t4062: F, t3994: F, t40763: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2550::<F>(t1389, t268, t2452, t40633, t547, t9793, t9794, t9930, t40634, t550, t9718, t247, t548, t9722);
        let (t46824, t46825, t46826, t46828, t46831, t46833) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2551::<F>(t1379, t40846, t550, t816, t1412, t9794, t1353, t1399, t9793, t40609, t4062, t3994, t40763);
    (t46810, t46812, t46817, t46820, t46824, t46825, t46826, t46828, t46831, t46833)
}
