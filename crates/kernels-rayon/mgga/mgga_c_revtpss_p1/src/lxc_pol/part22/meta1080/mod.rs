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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1080(t2661: f64, t74026: f64, t9835: f64, t9934: f64, t22016: f64, t22025: f64, t46609: f64, t6846: f64, t9909: f64, t1399: f64, t22236: f64, t3992: f64, t48533: f64, t6869: f64, t14045: f64, t22096: f64, t21990: f64, t5608: f64, t1353: f64, t13804: f64, t13805: f64, t1410: f64, t21969: f64, t22074: f64, t22079: f64, t3924: f64, t3934: f64, t3936: f64, t4012: f64, t47259: f64, t47262: f64, t5673: f64, t828: f64, t124: f64, t3944: f64, t47298: f64, t47304: f64, t49049: f64, t49053: f64, t49056: f64, t49058: f64, t49062: f64, t49066: f64, t49070: f64, t73345: f64, t800: f64, t1413: f64, t46835: f64, t74483: f64, t22061: f64, t9793: f64, t9794: f64, t22093: f64, t9962: f64, t13845: f64, t73731: f64, t9818: f64, t13847: f64, t13848: f64, t13921: f64, t22046: f64, t22118: f64, t4057: f64, t49085: f64, t49087: f64, t49090: f64, t49103: f64, t5674: f64, t9955: f64, t22026: f64, t46802: f64, t46694: f64, t6850: f64, t13783: f64, t13867: f64, t13872: f64, t1883: f64, t221: f64, t47320: f64, t49093: f64, t49105: f64, t49118: f64, t49121: f64, t49124: f64, t5591: f64, t5627: f64, t5659: f64, t22294: f64, t48823: f64, t9816: f64, t1398: f64, t6843: f64, t22245: f64, t808: f64, t9736: f64, t6884: f64, t9741: f64, t13789: f64, t3938: f64, t47337: f64, t47338: f64, t49126: f64, t49128: f64, t49134: f64, t49139: f64, t49144: f64, t73752: f64, t73791: f64, t73817: f64, t73870: f64, t73914: f64, t73947: f64, t73973: f64, t74004: f64, t74176: f64, t74215: f64, t74234: f64, t74266: f64, t74298: f64, t74329: f64, t74347: f64, t74375: f64, t74390: f64, t74397: f64, t74418: f64, t74441: f64, t74458: f64, t74496: f64, t74513: f64, t74527: f64, t74542: f64, t74558: f64, t74574: f64, t14104: f64, t47856: f64, t13729: f64, t2782: f64, t556: f64, t5774: f64, t1424: f64, t213: f64, t225: f64, t4077: f64, t47904: f64, t47907: f64, t47913: f64, t47918: f64, t47920: f64, t47926: f64, t47929: f64, t47932: f64, t47936: f64, t47938: f64, t47942: f64, t47944: f64, t561: f64, t6918: f64, t73705: f64, t73707: f64, t73712: f64, t9657: f64, t2439: f64, t3895: f64, t6896: f64, t14110: f64, t49471: f64, t136: f64, t2457: f64, t47480: f64, t6895: f64, t22414: f64, t686: f64, t72: f64, t9680: f64, t13739: f64, t13743: f64, t13746: f64, t22387: f64, t22395: f64, t4071: f64, t4131: f64, t47561: f64, t47568: f64, t47793: f64, t47794: f64, t47947: f64, t47952: f64, t49468: f64, t49472: f64, t49474: f64, t49476: f64, t49480: f64, t5715: f64, t22386: f64, t3915: f64, t49503: f64, t5722: f64, t22307: f64, t1358: f64, t6888: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74579, t74583, t74585, t74589) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3882(t2661, t74026, t9835, t9934, t22016, t22025, t46609, t6846, t9909, t1399, t22236, t3992);
        let t74616 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3883(t2661, t3992, t48533, t6869, t14045, t22096, t21990, t5608, t9934, t1353, t13804, t13805, t1410, t21969, t22074, t22079, t3924, t3934, t3936, t4012, t47259, t47262, t5673, t74579, t74583, t74585, t74589, t828);
        let t74636 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3884(t124, t22079, t3924, t3934, t3944, t47298, t47304, t49049, t49053, t49056, t49058, t49062, t49066, t49070, t5673, t73345, t800);
        let (t74638, t74641, t74656, t74660) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3885(t1413, t46835, t74483, t22061, t9793, t9794, t22093, t9962, t13845, t73731, t9818, t9835);
        let t74669 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3886(t13845, t13847, t13848, t21990, t13921, t22046, t22118, t3934, t4057, t49085, t49087, t49090, t49103, t5673, t5674, t74638, t74641, t74656, t74660, t9955);
        let t74696 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3887(t22026, t46802, t9794, t46694, t6850, t13783, t13867, t13872, t1883, t221, t3934, t47320, t49093, t49105, t49118, t49121, t49124, t5591, t5627, t5659);
        let (t74700, t74719) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3888(t22294, t48823, t9816, t1398, t6843, t22245, t808, t9736, t22236, t6884, t9741, t13789, t3934, t3938, t47337, t47338, t49126, t49128, t49134, t49139, t49144);
        let t74724 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3889(t73752, t73791, t73817, t73870, t73914, t73947, t73973, t74004, t74176, t74215, t74234, t74266, t74298, t74329, t74347, t74375, t74390, t74397, t74418, t74441, t74458, t74496, t74513, t74527, t74542, t74558, t74574, t74616, t74636, t74669, t74696, t74719);
        let t74749 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3890(t14104, t47856, t13729, t2782, t556, t5774, t1424, t213, t225, t4077, t47904, t47907, t47913, t47918, t47920, t47926, t47929, t47932, t47936, t47938, t47942, t47944, t561, t6918, t73705, t73707, t73712, t74724, t9657);
        let (t74757, t74763, t74770, t74782) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3891(t2439, t3895, t6896, t14110, t49471, t136, t2457, t47480, t6895, t22414, t686, t72, t9680);
        let t74786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3892(t13739, t13743, t13746, t1424, t22387, t22395, t4071, t4131, t47561, t47568, t47793, t47794, t47947, t47952, t49468, t49472, t49474, t49476, t49480, t5715, t6895, t74757, t74763, t74770, t74782, t9657);
        let (t74794, t74797, t74802, t74807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3893(t22386, t3915, t686, t72, t49503, t5722, t213, t22307, t1358, t2439, t6888, t785);
    (t74700, t74724, t74749, t74786, t74794, t74797, t74802, t74807)
}
