//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta624 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2214;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2215;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2216;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2217;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2218;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2219;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2220;
use chunk7::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2221;
use chunk8::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta624(t1659: f64, t25576: f64, t27489: f64, t3111: f64, t11940: f64, t7131: f64, t16158: f64, t7132: f64, t1068: f64, t15719: f64, t1675: f64, t25577: f64, t3101: f64, t3204: f64, t4831: f64, t4839: f64, t93618: f64, t93620: f64, t93622: f64, t93627: f64, t93675: f64, t100007: f64, t16094: f64, t12167: f64, t99984: f64, t12078: f64, t25516: f64, t4954: f64, t15752: f64, t27498: f64, t15596: f64, t15601: f64, t15615: f64, t15910: f64, t15965: f64, t16084: f64, t16128: f64, t16201: f64, t25517: f64, t3097: f64, t4788: f64, t4907: f64, t93670: f64, t93821: f64, t15734: f64, t25522: f64, t15816: f64, t7121: f64, t15822: f64, t25504: f64, t15794: f64, t25580: f64, t1047: f64, t15959: f64, t16104: f64, t27450: f64, t3136: f64, t3157: f64, t4783: f64, t4825: f64, t93646: f64, t93673: f64, t93683: f64, t93685: f64, t93752: f64, t11788: f64, t15787: f64, t15839: f64, t15895: f64, t15899: f64, t15922: f64, t16045: f64, t16098: f64, t16154: f64, t27493: f64, t27536: f64, t3177: f64, t3184: f64, t93543: f64, t93548: f64, t93658: f64, t4797: f64, t15970: f64, t93597: f64, t93687: f64, t93689: f64, t93694: f64, t93696: f64, t93702: f64, t93704: f64, t93713: f64, t93718: f64, t93720: f64, t15682: f64, t15811: f64, t16040: f64, t16078: f64, t25569: f64, t4803: f64, t4808: f64, t93743: f64, t93745: f64, t93750: f64, t93755: f64, t4857: f64, t16163: f64, t7122: f64, t15772: f64, t15984: f64, t1058: f64, t27464: f64, t3201: f64, t7801: f64, t27467: f64, t15887: f64, t16186: f64, t1972: f64, t25526: f64, t3130: f64, t375: f64, t4869: f64, t4875: f64, t7125: f64, t93764: f64, t15775: f64, t100054: f64, t3299: f64, t100030: f64, t15158: f64, t15586: f64, t15611: f64, t15697: f64, t16027: f64, t16123: f64, t16223: f64, t16230: f64, t25553: f64, t27526: f64, t27527: f64, t7111: f64, t93667: f64, t93799: f64, t93801: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t100133 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2214(t1659, t25576, t27489, t3111, t11940, t7131, t16158, t7132, t1068, t15719, t1675, t25577, t3101, t3204, t4831, t4839, t93618, t93620, t93622, t93627, t93675);
        let (t100135, t100163) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2215(t100007, t16094, t12167, t99984, t12078, t25516, t4954, t15752, t27498, t15596, t15601, t15615, t15910, t15965, t16084, t16128, t16201, t25517, t3097, t4788, t4907, t7132, t93670, t93821);
        let t100187 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2216(t15734, t25522, t15816, t7121, t15822, t25504, t15794, t25580, t1047, t15959, t16104, t25517, t27450, t3136, t3157, t4783, t4825, t93646, t93673, t93683, t93685, t93752, t93821);
        let t100216 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2217(t100135, t11788, t15787, t15839, t15895, t15899, t15922, t16045, t16098, t16154, t25580, t27489, t27493, t27536, t3177, t3184, t4839, t4907, t7131, t93543, t93548, t93658);
        let t100233 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2218(t4797, t7131, t1068, t15970, t27493, t4788, t93597, t93687, t93689, t93694, t93696, t93702, t93704, t93713, t93718, t93720);
        let t100254 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2219(t15682, t25517, t15811, t16040, t16078, t25522, t25569, t25577, t25580, t4803, t4808, t93743, t93745, t93750, t93755);
        let (t100255, t100261, t100262, t100268, t100270, t100272) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2220(t4857, t7131, t16163, t7122, t15772, t7132, t15984, t25517, t1058, t27464, t3201, t7801);
        let t100282 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2221(t1058, t27467, t100255, t100261, t100262, t100268, t100270, t100272, t15887, t16186, t1972, t25526, t3130, t375, t4797, t4869, t4875, t7122, t7125, t93764);
        let t100310 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2222(t15775, t7132, t100054, t3299, t100030, t15158, t15586, t15611, t15697, t16027, t16123, t16223, t16230, t1659, t25553, t27526, t27527, t375, t7111, t93658, t93667, t93752, t93799, t93801);
    (t100133, t100163, t100187, t100216, t100233, t100254, t100282, t100310)
}
