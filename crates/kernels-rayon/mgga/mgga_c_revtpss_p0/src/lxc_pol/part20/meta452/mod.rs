//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1724;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1725;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta452(t1386: f64, t2482: f64, t2668: f64, t9821: f64, t13999: f64, t9842: f64, t9828: f64, t9962: f64, t124: f64, t3923: f64, t3938: f64, t9816: f64, t9818: f64, t9769: f64, t9793: f64, t9794: f64, t1376: f64, t40757: f64, t2681: f64, t4000: f64, t820: f64, t4006: f64, t1399: f64, t2661: f64, t3992: f64, t9929: f64, t1412: f64, t4056: f64, t9810: f64, t9979: f64, t1388: f64, t1390: f64, t3934: f64, t3944: f64, t4002: f64, t46298: f64, t46547: f64, t46574: f64, t46628: f64, t46719: f64, t46723: f64, t46730: f64, t800: f64, t828: f64, t9826: f64, t9955: f64, t9956: f64, t10111: f64, t1408: f64, t9720: f64, t1353: f64, t1414: f64, t685: f64, t9770: f64, t9775: f64, t46610: f64, t543: f64, t4003: f64, t9934: f64, t40735: f64, t535: f64, t235: f64, t5744: f64, t2453: f64, t9935: f64, t1389: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46741, t46747, t46749, t46751, t46754) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1724(t1386, t2482, t2668, t9821, t13999, t9842, t9828, t9962, t124, t3923, t3938, t9816, t9818);
        let (t46757, t46760, t46767, t46771) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1725(t9769, t9793, t9794, t1376, t40757, t2681, t4000, t820, t4006, t1399, t2661, t3992, t9929);
        let t46782 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1726(t1412, t2661, t3938, t3992, t4056, t9810, t9979, t124, t1388, t1390, t3934, t3944, t4002, t46298, t46547, t46574, t46628, t46719, t46723, t46730, t46741, t46747, t46749, t46754, t46757, t46760, t46767, t46771, t800, t828, t9826, t9955, t9956);
        let (t46786, t46787, t46789, t46793) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1727(t10111, t1408, t9720, t1353, t1414, t685, t9770, t9775, t2661, t3992, t46610, t543);
        let (t46797, t46800, t46804, t46808) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1728(t2661, t4003, t46610, t9934, t40735, t535, t235, t5744, t2453, t9794, t9935, t1389, t268);
    (t46751, t46782, t46786, t46787, t46789, t46793, t46797, t46800, t46804, t46808)
}
