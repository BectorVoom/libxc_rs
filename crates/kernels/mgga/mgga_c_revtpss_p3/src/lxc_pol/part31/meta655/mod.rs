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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta655<F: Float>(t1032: F, t6888: F, t1426: F, t7063: F, t7286: F, t1955: F, t30016: F, t686: F, t72: F, t94674: F, t94669: F, t1358: F, t212: F, t30055: F, t689: F, t30056: F, t7289: F, t1444: F, t22433: F, t25921: F, t25924: F, t27837: F, t27903: F, t30017: F, t30020: F, t30021: F, t30101: F, t7279: F, t7295: F, t7304: F, t94662: F, t97843: F, t97847: F, t1398: F, t14224: F, t25930: F, t543: F, t7301: F, t94677: F, t94682: F, t97869: F, t97882: F, t97894: F, t97900: F, t97908: F, t97915: F, t97917: F, t97920: F, t97923: F, t97926: F, t98340: F, t7284: F, t30100: F, t25904: F, t25899: F, t27853: F, t27858: F, t27864: F, t5774: F, t7920: F, t94700: F, t94703: F, t94714: F, t94726: F, t94733: F, t94823: F, t97943: F, t97945: F, t97949: F, t30031: F, t25878: F, t22307: F, t1903: F, t2030: F, t26084: F, t27960: F, t6896: F, t7296: F, t7910: F, t94735: F, t94756: F, t94758: F, t97951: F, t97953: F, t97956: F, t97964: F, t97968: F, t97974: F, t786: F, t27989: F, t97802: F, t213: F, t1445: F, t27896: F, t27909: F, t5775: F, t94656: F, t94761: F, t94772: F, t94777: F, t94779: F, t97875: F, t97976: F, t97985: F, t98001: F, t98003: F, t7242: F, t22399: F, t26054: F, t27841: F, t27972: F, t6843: F, t7274: F, t7298: F, t7921: F, t94784: F, t94807: F, t94820: F, t94842: F, t98010: F, t98011: F, t98029: F, t98050: F, t27888: F, t27899: F, t27884: F, t27873: F, t97700: F, t98041: F, t22387: F, t22415: F, t28012: F, t7917: F, t94851: F, t94854: F, t94857: F, t98043: F, t98069: F, t98071: F, t98078: F, t98081: F, t6874: F, t22453: F, t94901: F, t25895: F, t108225: F, t14230: F, t25931: F, t27868: F, t27973: F, t27981: F, t3999: F, t6918: F, t75012: F, t94865: F, t94867: F, t97933: F, t98084: F, t98089: F, t98091: F, t98099: F, t108187: F, t6861: F, t30081: F, t94768: F, t94763: F, t5722: F, t97783: F, t2022: F, t22252: F, t30057: F, t30089: F, t7292: F, t94876: F, t98101: F, t98104: F, t98305: F, t98310: F, t98312: F, t98314: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t108278, t108280, t108282, t108294, t108296, t108302) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2195::<F>(t1032, t6888, t1426, t7063, t7286, t1955, t30016, t686, t72, t94674, t94669, t1358, t212, t30055, t689);
        let (t108307, t108310) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2196::<F>(t30056, t686, t72, t7289, t108280, t108282, t108294, t108296, t108302, t1444, t22433, t25921, t25924, t27837, t27903, t30017, t30020, t30021, t30101, t7279, t7295, t7304, t94662, t97843, t97847);
        let t108327 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2197::<F>(t1398, t14224, t25930, t30055, t543, t7295, t7301, t94677, t94682, t97869, t97882, t97894, t97900, t97908, t97915, t97917, t97920, t97923, t97926, t98340);
        let t108349 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2198::<F>(t108307, t7284, t30100, t689, t25904, t25899, t25924, t27837, t27853, t27858, t27864, t5774, t7295, t7920, t94700, t94703, t94714, t94726, t94733, t94823, t97943, t97945, t97949, t98340);
        let (t108368, t108374) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2199::<F>(t30031, t686, t72, t25878, t1955, t22307, t1903, t2030, t26084, t27960, t5774, t6896, t7295, t7296, t7910, t94735, t94756, t94758, t97951, t97953, t97956, t97964, t97968, t97974);
        let t108399 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2200::<F>(t108278, t786, t7286, t27989, t97802, t213, t30055, t1444, t1445, t25930, t27837, t27864, t27896, t27909, t30016, t5775, t7295, t94656, t94761, t94772, t94777, t94779, t97875, t97976, t97985, t98001, t98003);
        let t108425 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2201::<F>(t689, t6896, t7242, t22399, t26054, t108282, t25930, t27837, t27841, t27972, t543, t6843, t7274, t7295, t7298, t7301, t7921, t94784, t94807, t94820, t94842, t97875, t98010, t98011, t98029, t98050);
        let t108443 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2202::<F>(t27888, t27899, t27884, t27873, t97700, t98041, t22387, t22415, t28012, t7279, t7917, t94851, t94854, t94857, t98043, t98069, t98071, t98078, t98081);
        let t108471 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2203::<F>(t1444, t6874, t22453, t94901, t108368, t25895, t108225, t14230, t25930, t25931, t27868, t27973, t27981, t3999, t6918, t7274, t7295, t7296, t75012, t7910, t94865, t94867, t97933, t98084, t98089, t98091, t98099);
        let (t108484, t108500) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2204::<F>(t108187, t25878, t6861, t7274, t30081, t689, t94768, t94763, t5722, t97783, t2022, t22252, t25921, t30057, t30089, t543, t7292, t7295, t7301, t94876, t98101, t98104, t98305, t98310, t98312, t98314);
    (t108310, t108327, t108349, t108374, t108399, t108425, t108443, t108471, t108484, t108500)
}
