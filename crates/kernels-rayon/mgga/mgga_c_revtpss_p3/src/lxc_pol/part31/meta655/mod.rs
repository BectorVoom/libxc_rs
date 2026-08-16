//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta655 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2195;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2196;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2197;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2198;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2199;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2200;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2201;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2202;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2203;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta655(t1032: f64, t6888: f64, t1426: f64, t7063: f64, t7286: f64, t1955: f64, t30016: f64, t686: f64, t72: f64, t94674: f64, t94669: f64, t1358: f64, t212: f64, t30055: f64, t689: f64, t30056: f64, t7289: f64, t1444: f64, t22433: f64, t25921: f64, t25924: f64, t27837: f64, t27903: f64, t30017: f64, t30020: f64, t30021: f64, t30101: f64, t7279: f64, t7295: f64, t7304: f64, t94662: f64, t97843: f64, t97847: f64, t1398: f64, t14224: f64, t25930: f64, t543: f64, t7301: f64, t94677: f64, t94682: f64, t97869: f64, t97882: f64, t97894: f64, t97900: f64, t97908: f64, t97915: f64, t97917: f64, t97920: f64, t97923: f64, t97926: f64, t98340: f64, t7284: f64, t30100: f64, t25904: f64, t25899: f64, t27853: f64, t27858: f64, t27864: f64, t5774: f64, t7920: f64, t94700: f64, t94703: f64, t94714: f64, t94726: f64, t94733: f64, t94823: f64, t97943: f64, t97945: f64, t97949: f64, t30031: f64, t25878: f64, t22307: f64, t1903: f64, t2030: f64, t26084: f64, t27960: f64, t6896: f64, t7296: f64, t7910: f64, t94735: f64, t94756: f64, t94758: f64, t97951: f64, t97953: f64, t97956: f64, t97964: f64, t97968: f64, t97974: f64, t786: f64, t27989: f64, t97802: f64, t213: f64, t1445: f64, t27896: f64, t27909: f64, t5775: f64, t94656: f64, t94761: f64, t94772: f64, t94777: f64, t94779: f64, t97875: f64, t97976: f64, t97985: f64, t98001: f64, t98003: f64, t7242: f64, t22399: f64, t26054: f64, t27841: f64, t27972: f64, t6843: f64, t7274: f64, t7298: f64, t7921: f64, t94784: f64, t94807: f64, t94820: f64, t94842: f64, t98010: f64, t98011: f64, t98029: f64, t98050: f64, t27888: f64, t27899: f64, t27884: f64, t27873: f64, t97700: f64, t98041: f64, t22387: f64, t22415: f64, t28012: f64, t7917: f64, t94851: f64, t94854: f64, t94857: f64, t98043: f64, t98069: f64, t98071: f64, t98078: f64, t98081: f64, t6874: f64, t22453: f64, t94901: f64, t25895: f64, t108225: f64, t14230: f64, t25931: f64, t27868: f64, t27973: f64, t27981: f64, t3999: f64, t6918: f64, t75012: f64, t94865: f64, t94867: f64, t97933: f64, t98084: f64, t98089: f64, t98091: f64, t98099: f64, t108187: f64, t6861: f64, t30081: f64, t94768: f64, t94763: f64, t5722: f64, t97783: f64, t2022: f64, t22252: f64, t30057: f64, t30089: f64, t7292: f64, t94876: f64, t98101: f64, t98104: f64, t98305: f64, t98310: f64, t98312: f64, t98314: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t108278, t108280, t108282, t108294, t108296, t108302) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2195(t1032, t6888, t1426, t7063, t7286, t1955, t30016, t686, t72, t94674, t94669, t1358, t212, t30055, t689);
        let (t108307, t108310) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2196(t30056, t686, t72, t7289, t108280, t108282, t108294, t108296, t108302, t1444, t22433, t25921, t25924, t27837, t27903, t30017, t30020, t30021, t30101, t7279, t7295, t7304, t94662, t97843, t97847);
        let t108327 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2197(t1398, t14224, t25930, t30055, t543, t7295, t7301, t94677, t94682, t97869, t97882, t97894, t97900, t97908, t97915, t97917, t97920, t97923, t97926, t98340);
        let t108349 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2198(t108307, t7284, t30100, t689, t25904, t25899, t25924, t27837, t27853, t27858, t27864, t5774, t7295, t7920, t94700, t94703, t94714, t94726, t94733, t94823, t97943, t97945, t97949, t98340);
        let (t108368, t108374) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2199(t30031, t686, t72, t25878, t1955, t22307, t1903, t2030, t26084, t27960, t5774, t6896, t7295, t7296, t7910, t94735, t94756, t94758, t97951, t97953, t97956, t97964, t97968, t97974);
        let t108399 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2200(t108278, t786, t7286, t27989, t97802, t213, t30055, t1444, t1445, t25930, t27837, t27864, t27896, t27909, t30016, t5775, t7295, t94656, t94761, t94772, t94777, t94779, t97875, t97976, t97985, t98001, t98003);
        let t108425 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2201(t689, t6896, t7242, t22399, t26054, t108282, t25930, t27837, t27841, t27972, t543, t6843, t7274, t7295, t7298, t7301, t7921, t94784, t94807, t94820, t94842, t97875, t98010, t98011, t98029, t98050);
        let t108443 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2202(t27888, t27899, t27884, t27873, t97700, t98041, t22387, t22415, t28012, t7279, t7917, t94851, t94854, t94857, t98043, t98069, t98071, t98078, t98081);
        let t108471 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2203(t1444, t6874, t22453, t94901, t108368, t25895, t108225, t14230, t25930, t25931, t27868, t27973, t27981, t3999, t6918, t7274, t7295, t7296, t75012, t7910, t94865, t94867, t97933, t98084, t98089, t98091, t98099);
        let (t108484, t108500) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2204(t108187, t25878, t6861, t7274, t30081, t689, t94768, t94763, t5722, t97783, t2022, t22252, t25921, t30057, t30089, t543, t7292, t7295, t7301, t94876, t98101, t98104, t98305, t98310, t98312, t98314);
    (t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108484, t108500)
}
