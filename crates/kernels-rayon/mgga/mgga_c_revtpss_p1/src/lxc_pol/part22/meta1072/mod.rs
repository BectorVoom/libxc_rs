//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1072 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3843;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3846;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1072(t4011: f64, t61999: f64, t246: f64, t5674: f64, t13783: f64, t13784: f64, t13789: f64, t13790: f64, t13791: f64, t1399: f64, t21990: f64, t22046: f64, t22279: f64, t22841: f64, t3934: f64, t4004: f64, t46596: f64, t46924: f64, t48073: f64, t48111: f64, t48466: f64, t49107: f64, t5671: f64, t5673: f64, t5675: f64, t5745: f64, t73847: f64, t9955: f64, t1412: f64, t6843: f64, t2661: f64, t3938: f64, t3992: f64, t22020: f64, t46766: f64, t6864: f64, t5658: f64, t543: f64, t4003: f64, t1388: f64, t1390: f64, t1410: f64, t3889: f64, t4002: f64, t4012: f64, t46598: f64, t46602: f64, t46620: f64, t46633: f64, t46645: f64, t6816: f64, t828: f64, t9934: f64, t22267: f64, t9976: f64, t13847: f64, t73731: f64, t9816: f64, t22074: f64, t3936: f64, t4057: f64, t48143: f64, t48445: f64, t48449: f64, t48453: f64, t48458: f64, t48462: f64, t9956: f64, t22294: f64, t48862: f64, t48999: f64, t22025: f64, t6836: f64, t9940: f64, t1353: f64, t13767: f64, t13768: f64, t5591: f64, t1414: f64, t22079: f64, t46649: f64, t46652: f64, t48486: f64, t48488: f64, t48494: f64, t48498: f64, t73578: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73906, t73908, t73914) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842(t4011, t61999, t246, t5674, t13783, t13784, t13789, t13790, t13791, t1399, t21990, t22046, t22279, t22841, t3934, t4004, t46596, t46924, t48073, t48111, t48466, t49107, t5671, t5673, t5675, t5745, t73847, t9955);
        let (t73923, t73927, t73929, t73937, t73942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3843(t1412, t6843, t2661, t3938, t3992, t1399, t22020, t46766, t6864, t5658, t543, t4003);
        let t73947 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844(t1388, t1390, t1410, t3889, t4002, t4012, t46598, t46602, t46620, t46633, t46645, t6816, t73923, t73927, t73929, t73937, t73942, t828);
        let t73973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845(t22020, t2661, t5675, t9934, t22267, t9976, t13847, t1399, t73731, t9816, t22046, t22074, t3934, t3936, t4057, t48143, t48445, t48449, t48453, t48458, t48462, t9955, t9956);
        let (t73975, t73985, t73991, t73994, t73998) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3846(t22294, t48862, t48999, t22025, t2661, t5675, t9934, t6836, t9940, t1353, t13767, t13768, t5591);
        let t74004 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847(t1410, t1414, t22079, t4004, t46649, t46652, t48486, t48488, t48494, t48498, t5671, t5673, t73578, t73975, t73985, t73994, t73998, t828);
    (t73906, t73908, t73914, t73937, t73942, t73947, t73973, t73991, t74004)
}
