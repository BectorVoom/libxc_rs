//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2550;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta715(t1389: f64, t268: f64, t2452: f64, t40633: f64, t547: f64, t9793: f64, t9794: f64, t9930: f64, t40634: f64, t550: f64, t9718: f64, t247: f64, t548: f64, t9722: f64, t1379: f64, t40846: f64, t816: f64, t1412: f64, t1353: f64, t1399: f64, t40609: f64, t4062: f64, t3994: f64, t40763: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46810, t46812, t46817, t46820) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2550(t1389, t268, t2452, t40633, t547, t9793, t9794, t9930, t40634, t550, t9718, t247, t548, t9722);
        let (t46824, t46825, t46826, t46828, t46831, t46833) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2551(t1379, t40846, t550, t816, t1412, t9794, t1353, t1399, t9793, t40609, t4062, t3994, t40763);
    (t46810, t46812, t46817, t46820, t46824, t46825, t46826, t46828, t46831, t46833)
}
