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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta624<F: Float>(t1659: F, t25576: F, t27489: F, t3111: F, t11940: F, t7131: F, t16158: F, t7132: F, t1068: F, t15719: F, t1675: F, t25577: F, t3101: F, t3204: F, t4831: F, t4839: F, t93618: F, t93620: F, t93622: F, t93627: F, t93675: F, t100007: F, t16094: F, t12167: F, t99984: F, t12078: F, t25516: F, t4954: F, t15752: F, t27498: F, t15596: F, t15601: F, t15615: F, t15910: F, t15965: F, t16084: F, t16128: F, t16201: F, t25517: F, t3097: F, t4788: F, t4907: F, t93670: F, t93821: F, t15734: F, t25522: F, t15816: F, t7121: F, t15822: F, t25504: F, t15794: F, t25580: F, t1047: F, t15959: F, t16104: F, t27450: F, t3136: F, t3157: F, t4783: F, t4825: F, t93646: F, t93673: F, t93683: F, t93685: F, t93752: F, t11788: F, t15787: F, t15839: F, t15895: F, t15899: F, t15922: F, t16045: F, t16098: F, t16154: F, t27493: F, t27536: F, t3177: F, t3184: F, t93543: F, t93548: F, t93658: F, t4797: F, t15970: F, t93597: F, t93687: F, t93689: F, t93694: F, t93696: F, t93702: F, t93704: F, t93713: F, t93718: F, t93720: F, t15682: F, t15811: F, t16040: F, t16078: F, t25569: F, t4803: F, t4808: F, t93743: F, t93745: F, t93750: F, t93755: F, t4857: F, t16163: F, t7122: F, t15772: F, t15984: F, t1058: F, t27464: F, t3201: F, t7801: F, t27467: F, t15887: F, t16186: F, t1972: F, t25526: F, t3130: F, t375: F, t4869: F, t4875: F, t7125: F, t93764: F, t15775: F, t100054: F, t3299: F, t100030: F, t15158: F, t15586: F, t15611: F, t15697: F, t16027: F, t16123: F, t16223: F, t16230: F, t25553: F, t27526: F, t27527: F, t7111: F, t93667: F, t93799: F, t93801: F) -> (F, F, F, F, F, F, F, F) {
        let t100133 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2214::<F>(t1659, t25576, t27489, t3111, t11940, t7131, t16158, t7132, t1068, t15719, t1675, t25577, t3101, t3204, t4831, t4839, t93618, t93620, t93622, t93627, t93675);
        let (t100135, t100163) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2215::<F>(t100007, t16094, t12167, t99984, t12078, t25516, t4954, t15752, t27498, t15596, t15601, t15615, t15910, t15965, t16084, t16128, t16201, t25517, t3097, t4788, t4907, t7132, t93670, t93821);
        let t100187 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2216::<F>(t15734, t25522, t15816, t7121, t15822, t25504, t15794, t25580, t1047, t15959, t16104, t25517, t27450, t3136, t3157, t4783, t4825, t93646, t93673, t93683, t93685, t93752, t93821);
        let t100216 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2217::<F>(t100135, t11788, t15787, t15839, t15895, t15899, t15922, t16045, t16098, t16154, t25580, t27489, t27493, t27536, t3177, t3184, t4839, t4907, t7131, t93543, t93548, t93658);
        let t100233 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2218::<F>(t4797, t7131, t1068, t15970, t27493, t4788, t93597, t93687, t93689, t93694, t93696, t93702, t93704, t93713, t93718, t93720);
        let t100254 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2219::<F>(t15682, t25517, t15811, t16040, t16078, t25522, t25569, t25577, t25580, t4803, t4808, t93743, t93745, t93750, t93755);
        let (t100255, t100261, t100262, t100268, t100270, t100272) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2220::<F>(t4857, t7131, t16163, t7122, t15772, t7132, t15984, t25517, t1058, t27464, t3201, t7801);
        let t100282 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2221::<F>(t1058, t27467, t100255, t100261, t100262, t100268, t100270, t100272, t15887, t16186, t1972, t25526, t3130, t375, t4797, t4869, t4875, t7122, t7125, t93764);
        let t100310 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2222::<F>(t15775, t7132, t100054, t3299, t100030, t15158, t15586, t15611, t15697, t16027, t16123, t16223, t16230, t1659, t25553, t27526, t27527, t375, t7111, t93658, t93667, t93752, t93799, t93801);
    (t100133, t100163, t100187, t100216, t100233, t100254, t100282, t100310)
}
