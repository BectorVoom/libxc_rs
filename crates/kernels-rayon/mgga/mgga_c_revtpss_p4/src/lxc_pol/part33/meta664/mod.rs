//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta664 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2163;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2164;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2165;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2166;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2167;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2168;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2169;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2170;
use chunk8::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2171;
use chunk9::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2172;
use chunk10::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta664(t1364: f64, t30074: f64, t786: f64, t1882: f64, t543: f64, t5774: f64, t30020: f64, t686: f64, t72: f64, t25895: f64, t1398: f64, t6918: f64, t25921: f64, t25930: f64, t25931: f64, t27837: f64, t27868: f64, t27980: f64, t28003: f64, t30032: f64, t30096: f64, t5658: f64, t7295: f64, t7301: f64, t75047: f64, t75051: f64, t75305: f64, t7910: f64, t7926: f64, t94602: f64, t97764: f64, t97785: f64, t98050: f64, t1955: f64, t27883: f64, t1444: f64, t25924: f64, t27865: f64, t27869: f64, t27909: f64, t30031: f64, t30106: f64, t5728: f64, t94608: f64, t94616: f64, t94705: f64, t97792: f64, t97795: f64, t97798: f64, t97800: f64, t97804: f64, t97808: f64, t97810: f64, t97815: f64, t97933: f64, t6844: f64, t30095: f64, t689: f64, t25904: f64, t25899: f64, t1903: f64, t14224: f64, t27846: f64, t27960: f64, t30055: f64, t30105: f64, t7296: f64, t94635: f64, t94648: f64, t94716: f64, t97823: f64, t97825: f64, t97838: f64, t97875: f64, t1032: f64, t6888: f64, t1426: f64, t7063: f64, t7286: f64, t30016: f64, t94674: f64, t94669: f64, t1358: f64, t212: f64, t30056: f64, t7289: f64, t22433: f64, t27903: f64, t30017: f64, t30021: f64, t30101: f64, t7279: f64, t7304: f64, t94662: f64, t97843: f64, t97847: f64, t94677: f64, t94682: f64, t97869: f64, t97882: f64, t97894: f64, t97900: f64, t97908: f64, t97915: f64, t97917: f64, t97920: f64, t97923: f64, t97926: f64, t98340: f64, t7284: f64, t30100: f64, t27853: f64, t27858: f64, t27864: f64, t7920: f64, t94700: f64, t94703: f64, t94714: f64, t94726: f64, t94733: f64, t94823: f64, t97943: f64, t97945: f64, t97949: f64, t25878: f64, t22307: f64, t2030: f64, t26084: f64, t6896: f64, t94735: f64, t94756: f64, t94758: f64, t97951: f64, t97953: f64, t97956: f64, t97964: f64, t97968: f64, t97974: f64, t27989: f64, t97802: f64, t213: f64, t1445: f64, t27896: f64, t5775: f64, t94656: f64, t94761: f64, t94772: f64, t94777: f64, t94779: f64, t97976: f64, t97985: f64, t98001: f64, t98003: f64, t7242: f64, t22399: f64, t26054: f64, t27841: f64, t27972: f64, t6843: f64, t7274: f64, t7298: f64, t7921: f64, t94784: f64, t94807: f64, t94820: f64, t94842: f64, t98010: f64, t98011: f64, t98029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108175, t108178, t108187, t108188, t108206) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2163(t1364, t30074, t786, t1882, t543, t5774, t30020, t686, t72, t25895, t1398, t6918);
        let t108213 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2164(t108175, t108178, t108188, t108206, t25921, t25930, t25931, t27837, t27868, t27980, t28003, t30032, t30096, t543, t5658, t7295, t7301, t75047, t75051, t75305, t7910, t7926, t94602, t97764, t97785, t98050);
        let (t108225, t108233) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2165(t1955, t27883, t1444, t25924, t27865, t27869, t27909, t30031, t30106, t5728, t7295, t94608, t94616, t94705, t97792, t97795, t97798, t97800, t97804, t97808, t97810, t97815, t97933);
        let t108270 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2166(t1444, t6844, t30095, t689, t25904, t25899, t1903, t543, t5658, t14224, t1882, t25930, t25931, t27837, t27846, t27868, t27960, t30055, t30105, t7295, t7296, t7301, t94635, t94648, t94716, t97823, t97825, t97838, t97875);
        let (t108278, t108280, t108282, t108294, t108296, t108302) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2167(t1032, t6888, t1426, t7063, t7286, t1955, t30016, t686, t72, t94674, t94669, t1358, t212, t30055, t689);
        let (t108307, t108310) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2168(t30056, t686, t72, t7289, t108280, t108282, t108294, t108296, t108302, t1444, t22433, t25921, t25924, t27837, t27903, t30017, t30020, t30021, t30101, t7279, t7295, t7304, t94662, t97843, t97847);
        let t108327 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2169(t1398, t14224, t25930, t30055, t543, t7295, t7301, t94677, t94682, t97869, t97882, t97894, t97900, t97908, t97915, t97917, t97920, t97923, t97926, t98340);
        let t108349 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2170(t108307, t7284, t30100, t689, t25904, t25899, t25924, t27837, t27853, t27858, t27864, t5774, t7295, t7920, t94700, t94703, t94714, t94726, t94733, t94823, t97943, t97945, t97949, t98340);
        let (t108368, t108374) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2171(t30031, t686, t72, t25878, t1955, t22307, t1903, t2030, t26084, t27960, t5774, t6896, t7295, t7296, t7910, t94735, t94756, t94758, t97951, t97953, t97956, t97964, t97968, t97974);
        let t108399 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2172(t108278, t786, t7286, t27989, t97802, t213, t30055, t1444, t1445, t25930, t27837, t27864, t27896, t27909, t30016, t5775, t7295, t94656, t94761, t94772, t94777, t94779, t97875, t97976, t97985, t98001, t98003);
        let t108425 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2173(t689, t6896, t7242, t22399, t26054, t108282, t25930, t27837, t27841, t27972, t543, t6843, t7274, t7295, t7298, t7301, t7921, t94784, t94807, t94820, t94842, t97875, t98010, t98011, t98029, t98050);
    (t108187, t108213, t108225, t108233, t108270, t108310, t108327, t108349, t108368, t108374, t108399, t108425)
}
