//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta797 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2894;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta797(t1353: f64, t1414: f64, t685: f64, t46784: f64, t9770: f64, t9775: f64, t40735: f64, t535: f64, t235: f64, t5744: f64, t2453: f64, t9794: f64, t9935: f64, t1389: f64, t268: f64, t2452: f64, t40633: f64, t547: f64, t9793: f64, t9930: f64, t40634: f64, t550: f64, t9718: f64, t247: f64, t548: f64, t9722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46786, t46787, t46789, t46800, t46801, t46802, t46804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2894(t1353, t1414, t685, t46784, t9770, t9775, t40735, t535, t235, t5744, t2453, t9794, t9935);
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2895(t1389, t268, t2452, t40633, t547, t9793, t9794, t9930, t40634, t550, t9718, t247, t548, t9722);
    (t46786, t46787, t46789, t46800, t46801, t46802, t46804, t46810, t46812, t46817, t46820)
}
