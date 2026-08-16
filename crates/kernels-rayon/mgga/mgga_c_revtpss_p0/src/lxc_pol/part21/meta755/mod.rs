//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta755(t1353: f64, t1883: f64, t46825: f64, t9793: f64, t13848: f64, t9810: f64, t9816: f64, t9818: f64, t1408: f64, t241: f64, t820: f64, t2482: f64, t814: f64, t9991: f64, t13805: f64, t13847: f64, t13789: f64, t13790: f64, t13804: f64, t13944: f64, t3924: f64, t3934: f64, t3936: f64, t4056: f64, t46800: f64, t46804: f64, t46810: f64, t47248: f64, t48595: f64, t48686: f64, t48687: f64, t48691: f64, t48692: f64, t48696: f64, t543: f64, t5671: f64, t5673: f64, t5674: f64, t5675: f64, t5704: f64, t9628: f64, t9840: f64, t9984: f64, t46917: f64, t5706: f64, t47201: f64, t46478: f64, t9898: f64, t13783: f64, t13926: f64, t13975: f64, t46812: f64, t46817: f64, t46820: f64, t46824: f64, t46828: f64, t46831: f64, t46833: f64, t46837: f64, t46840: f64, t46846: f64, t46853: f64, t46859: f64, t47249: f64, t9956: f64, t9995: f64, t2661: f64, t3992: f64, t4057: f64, t5608: f64, t4004: f64, t5651: f64, t9934: f64, t47198: f64, t5665: f64, t5629: f64, t9779: f64, t5661: f64, t9909: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48698, t48700, t48709, t48712, t48731) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2647(t1353, t1883, t46825, t9793, t13848, t9810, t9816, t9818, t1408, t241, t820, t2482, t814, t9991);
        let t48745 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648(t13805, t13847, t13848, t48731, t1353, t13789, t13790, t13804, t13944, t3924, t3934, t3936, t4056, t46800, t46804, t46810, t47248, t48595, t48686, t48687, t48691, t48692, t48696, t48700, t48709, t48712, t543, t5671, t5673, t5674, t5675, t5704, t9628, t9840, t9984);
        let (t48760, t48778) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2649(t46917, t5706, t241, t47201, t820, t46478, t9898, t13783, t13804, t13926, t13975, t3924, t3934, t3936, t46812, t46817, t46820, t46824, t46828, t46831, t46833, t46837, t46840, t46846, t46853, t46859, t47248, t47249, t5673, t5674, t9956, t9995);
        let (t48786, t48790, t48792, t48794, t48796) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2650(t2661, t3992, t4057, t5608, t4004, t5651, t9934, t47198, t5665, t5629, t9779, t5661, t9909);
    (t48698, t48745, t48760, t48778, t48786, t48790, t48792, t48794, t48796)
}
