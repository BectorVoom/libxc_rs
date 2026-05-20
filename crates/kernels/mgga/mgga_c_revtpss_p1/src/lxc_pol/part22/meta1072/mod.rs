//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1072 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3843;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3846;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1072<F: Float>(t4011: F, t61999: F, t246: F, t5674: F, t13783: F, t13784: F, t13789: F, t13790: F, t13791: F, t1399: F, t21990: F, t22046: F, t22279: F, t22841: F, t3934: F, t4004: F, t46596: F, t46924: F, t48073: F, t48111: F, t48466: F, t49107: F, t5671: F, t5673: F, t5675: F, t5745: F, t73847: F, t9955: F, t1412: F, t6843: F, t2661: F, t3938: F, t3992: F, t22020: F, t46766: F, t6864: F, t5658: F, t543: F, t4003: F, t1388: F, t1390: F, t1410: F, t3889: F, t4002: F, t4012: F, t46598: F, t46602: F, t46620: F, t46633: F, t46645: F, t6816: F, t828: F, t9934: F, t22267: F, t9976: F, t13847: F, t73731: F, t9816: F, t22074: F, t3936: F, t4057: F, t48143: F, t48445: F, t48449: F, t48453: F, t48458: F, t48462: F, t9956: F, t22294: F, t48862: F, t48999: F, t22025: F, t6836: F, t9940: F, t1353: F, t13767: F, t13768: F, t5591: F, t1414: F, t22079: F, t46649: F, t46652: F, t48486: F, t48488: F, t48494: F, t48498: F, t73578: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t73906, t73908, t73914) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3842::<F>(t4011, t61999, t246, t5674, t13783, t13784, t13789, t13790, t13791, t1399, t21990, t22046, t22279, t22841, t3934, t4004, t46596, t46924, t48073, t48111, t48466, t49107, t5671, t5673, t5675, t5745, t73847, t9955);
        let (t73923, t73927, t73929, t73937, t73942) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3843::<F>(t1412, t6843, t2661, t3938, t3992, t1399, t22020, t46766, t6864, t5658, t543, t4003);
        let t73947 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844::<F>(t1388, t1390, t1410, t3889, t4002, t4012, t46598, t46602, t46620, t46633, t46645, t6816, t73923, t73927, t73929, t73937, t73942, t828);
        let t73973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3845::<F>(t22020, t2661, t5675, t9934, t22267, t9976, t13847, t1399, t73731, t9816, t22046, t22074, t3934, t3936, t4057, t48143, t48445, t48449, t48453, t48458, t48462, t9955, t9956);
        let (t73975, t73985, t73991, t73994, t73998) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3846::<F>(t22294, t48862, t48999, t22025, t2661, t5675, t9934, t6836, t9940, t1353, t13767, t13768, t5591);
        let t74004 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3847::<F>(t1410, t1414, t22079, t4004, t46649, t46652, t48486, t48488, t48494, t48498, t5671, t5673, t73578, t73975, t73985, t73994, t73998, t828);
    (t73906, t73908, t73914, t73937, t73942, t73947, t73973, t73991, t74004)
}
