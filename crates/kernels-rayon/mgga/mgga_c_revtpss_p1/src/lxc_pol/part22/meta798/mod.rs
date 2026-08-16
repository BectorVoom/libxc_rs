//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta798 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2896;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2897;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta798(t1379: f64, t40846: f64, t550: f64, t816: f64, t1412: f64, t9794: f64, t1353: f64, t1399: f64, t9793: f64, t40609: f64, t4062: f64, t3994: f64, t40763: f64, t2735: f64, t9792: f64, t1413: f64, t1376: f64, t40769: f64, t10111: f64, t1386: f64, t9720: f64, t1390: f64, t685: f64, t9970: f64, t9976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46824, t46825, t46826, t46828, t46831, t46833) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2896(t1379, t40846, t550, t816, t1412, t9794, t1353, t1399, t9793, t40609, t4062, t3994, t40763);
        let t46835 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2897(t2735, t9792);
        let (t46837, t46840, t46856, t46859, t46861) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2898(t1413, t46826, t46835, t1376, t40769, t10111, t1386, t9720, t1390, t1399, t685, t9970, t9976);
    (t46824, t46825, t46828, t46831, t46833, t46835, t46837, t46840, t46856, t46859, t46861)
}
