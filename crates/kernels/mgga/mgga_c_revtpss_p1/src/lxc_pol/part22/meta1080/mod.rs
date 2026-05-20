//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1080 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3882;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3883;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3884;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3885;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3886;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3887;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3888;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3889;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3890;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3891;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3892;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1080<F: Float>(t2661: F, t74026: F, t9835: F, t9934: F, t22016: F, t22025: F, t46609: F, t6846: F, t9909: F, t1399: F, t22236: F, t3992: F, t48533: F, t6869: F, t14045: F, t22096: F, t21990: F, t5608: F, t1353: F, t13804: F, t13805: F, t1410: F, t21969: F, t22074: F, t22079: F, t3924: F, t3934: F, t3936: F, t4012: F, t47259: F, t47262: F, t5673: F, t828: F, t124: F, t3944: F, t47298: F, t47304: F, t49049: F, t49053: F, t49056: F, t49058: F, t49062: F, t49066: F, t49070: F, t73345: F, t800: F, t1413: F, t46835: F, t74483: F, t22061: F, t9793: F, t9794: F, t22093: F, t9962: F, t13845: F, t73731: F, t9818: F, t13847: F, t13848: F, t13921: F, t22046: F, t22118: F, t4057: F, t49085: F, t49087: F, t49090: F, t49103: F, t5674: F, t9955: F, t22026: F, t46802: F, t46694: F, t6850: F, t13783: F, t13867: F, t13872: F, t1883: F, t221: F, t47320: F, t49093: F, t49105: F, t49118: F, t49121: F, t49124: F, t5591: F, t5627: F, t5659: F, t22294: F, t48823: F, t9816: F, t1398: F, t6843: F, t22245: F, t808: F, t9736: F, t6884: F, t9741: F, t13789: F, t3938: F, t47337: F, t47338: F, t49126: F, t49128: F, t49134: F, t49139: F, t49144: F, t73752: F, t73791: F, t73817: F, t73870: F, t73914: F, t73947: F, t73973: F, t74004: F, t74176: F, t74215: F, t74234: F, t74266: F, t74298: F, t74329: F, t74347: F, t74375: F, t74390: F, t74397: F, t74418: F, t74441: F, t74458: F, t74496: F, t74513: F, t74527: F, t74542: F, t74558: F, t74574: F, t14104: F, t47856: F, t13729: F, t2782: F, t556: F, t5774: F, t1424: F, t213: F, t225: F, t4077: F, t47904: F, t47907: F, t47913: F, t47918: F, t47920: F, t47926: F, t47929: F, t47932: F, t47936: F, t47938: F, t47942: F, t47944: F, t561: F, t6918: F, t73705: F, t73707: F, t73712: F, t9657: F, t2439: F, t3895: F, t6896: F, t14110: F, t49471: F, t136: F, t2457: F, t47480: F, t6895: F, t22414: F, t686: F, t72: F, t9680: F, t13739: F, t13743: F, t13746: F, t22387: F, t22395: F, t4071: F, t4131: F, t47561: F, t47568: F, t47793: F, t47794: F, t47947: F, t47952: F, t49468: F, t49472: F, t49474: F, t49476: F, t49480: F, t5715: F, t22386: F, t3915: F, t49503: F, t5722: F, t22307: F, t1358: F, t6888: F, t785: F) -> (F, F, F, F, F, F, F, F) {
        let (t74579, t74583, t74585, t74589) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3882::<F>(t2661, t74026, t9835, t9934, t22016, t22025, t46609, t6846, t9909, t1399, t22236, t3992);
        let t74616 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3883::<F>(t2661, t3992, t48533, t6869, t14045, t22096, t21990, t5608, t9934, t1353, t13804, t13805, t1410, t21969, t22074, t22079, t3924, t3934, t3936, t4012, t47259, t47262, t5673, t74579, t74583, t74585, t74589, t828);
        let t74636 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3884::<F>(t124, t22079, t3924, t3934, t3944, t47298, t47304, t49049, t49053, t49056, t49058, t49062, t49066, t49070, t5673, t73345, t800);
        let (t74638, t74641, t74656, t74660) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3885::<F>(t1413, t46835, t74483, t22061, t9793, t9794, t22093, t9962, t13845, t73731, t9818, t9835);
        let t74669 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3886::<F>(t13845, t13847, t13848, t21990, t13921, t22046, t22118, t3934, t4057, t49085, t49087, t49090, t49103, t5673, t5674, t74638, t74641, t74656, t74660, t9955);
        let t74696 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3887::<F>(t22026, t46802, t9794, t46694, t6850, t13783, t13867, t13872, t1883, t221, t3934, t47320, t49093, t49105, t49118, t49121, t49124, t5591, t5627, t5659);
        let (t74700, t74719) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3888::<F>(t22294, t48823, t9816, t1398, t6843, t22245, t808, t9736, t22236, t6884, t9741, t13789, t3934, t3938, t47337, t47338, t49126, t49128, t49134, t49139, t49144);
        let t74724 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3889::<F>(t73752, t73791, t73817, t73870, t73914, t73947, t73973, t74004, t74176, t74215, t74234, t74266, t74298, t74329, t74347, t74375, t74390, t74397, t74418, t74441, t74458, t74496, t74513, t74527, t74542, t74558, t74574, t74616, t74636, t74669, t74696, t74719);
        let t74749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3890::<F>(t14104, t47856, t13729, t2782, t556, t5774, t1424, t213, t225, t4077, t47904, t47907, t47913, t47918, t47920, t47926, t47929, t47932, t47936, t47938, t47942, t47944, t561, t6918, t73705, t73707, t73712, t74724, t9657);
        let (t74757, t74763, t74770, t74782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3891::<F>(t2439, t3895, t6896, t14110, t49471, t136, t2457, t47480, t6895, t22414, t686, t72, t9680);
        let t74786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3892::<F>(t13739, t13743, t13746, t1424, t22387, t22395, t4071, t4131, t47561, t47568, t47793, t47794, t47947, t47952, t49468, t49472, t49474, t49476, t49480, t5715, t6895, t74757, t74763, t74770, t74782, t9657);
        let (t74794, t74797, t74802, t74807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3893::<F>(t22386, t3915, t686, t72, t49503, t5722, t213, t22307, t1358, t2439, t6888, t785);
    (t74700, t74724, t74749, t74786, t74794, t74797, t74802, t74807)
}
