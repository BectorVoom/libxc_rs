//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta797 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2894;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta797<F: Float>(t1353: F, t1414: F, t685: F, t46784: F, t9770: F, t9775: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t9794: F, t9935: F, t1389: F, t268: F, t2452: F, t40633: F, t547: F, t9793: F, t9930: F, t40634: F, t550: F, t9718: F, t247: F, t548: F, t9722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t46786, t46787, t46789, t46800, t46801, t46802, t46804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2894::<F>(t1353, t1414, t685, t46784, t9770, t9775, t40735, t535, t235, t5744, t2453, t9794, t9935);
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2895::<F>(t1389, t268, t2452, t40633, t547, t9793, t9794, t9930, t40634, t550, t9718, t247, t548, t9722);
    (t46786, t46787, t46789, t46800, t46801, t46802, t46804, t46810, t46812, t46817, t46820)
}
