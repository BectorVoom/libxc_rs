//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1724;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1725;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta452<F: Float>(t1386: F, t2482: F, t2668: F, t9821: F, t13999: F, t9842: F, t9828: F, t9962: F, t124: F, t3923: F, t3938: F, t9816: F, t9818: F, t9769: F, t9793: F, t9794: F, t1376: F, t40757: F, t2681: F, t4000: F, t820: F, t4006: F, t1399: F, t2661: F, t3992: F, t9929: F, t1412: F, t4056: F, t9810: F, t9979: F, t1388: F, t1390: F, t3934: F, t3944: F, t4002: F, t46298: F, t46547: F, t46574: F, t46628: F, t46719: F, t46723: F, t46730: F, t800: F, t828: F, t9826: F, t9955: F, t9956: F, t10111: F, t1408: F, t9720: F, t1353: F, t1414: F, t685: F, t9770: F, t9775: F, t46610: F, t543: F, t4003: F, t9934: F, t40735: F, t535: F, t235: F, t5744: F, t2453: F, t9935: F, t1389: F, t268: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46741, t46747, t46749, t46751, t46754) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1724::<F>(t1386, t2482, t2668, t9821, t13999, t9842, t9828, t9962, t124, t3923, t3938, t9816, t9818);
        let (t46757, t46760, t46767, t46771) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1725::<F>(t9769, t9793, t9794, t1376, t40757, t2681, t4000, t820, t4006, t1399, t2661, t3992, t9929);
        let t46782 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726::<F>(t1412, t2661, t3938, t3992, t4056, t9810, t9979, t124, t1388, t1390, t3934, t3944, t4002, t46298, t46547, t46574, t46628, t46719, t46723, t46730, t46741, t46747, t46749, t46754, t46757, t46760, t46767, t46771, t800, t828, t9826, t9955, t9956);
        let (t46786, t46787, t46789, t46793) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727::<F>(t10111, t1408, t9720, t1353, t1414, t685, t9770, t9775, t2661, t3992, t46610, t543);
        let (t46797, t46800, t46804, t46808) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1728::<F>(t2661, t4003, t46610, t9934, t40735, t535, t235, t5744, t2453, t9794, t9935, t1389, t268);
    (t46751, t46782, t46786, t46787, t46789, t46793, t46797, t46800, t46804, t46808)
}
