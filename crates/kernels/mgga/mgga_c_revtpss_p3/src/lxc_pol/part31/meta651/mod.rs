//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta651 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2152;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2153;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2154;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2155;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2156;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2157;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2158;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2159;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2160;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2161;
use chunk10::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2162;
use chunk11::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2163;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta651<F: Float>(t19696: F, t7121: F, t20016: F, t25500: F, t19463: F, t1972: F, t100030: F, t100302: F, t100345: F, t1028: F, t1665: F, t19770: F, t19940: F, t19993: F, t19998: F, t25490: F, t25522: F, t27479: F, t4854: F, t6278: F, t6339: F, t7117: F, t93720: F, t93728: F, t19976: F, t25580: F, t19900: F, t7111: F, t19718: F, t19831: F, t19973: F, t19982: F, t20070: F, t20075: F, t20091: F, t27493: F, t27498: F, t93658: F, t93667: F, t93745: F, t93750: F, t1058: F, t29779: F, t100146: F, t100240: F, t100261: F, t100262: F, t100268: F, t100270: F, t100272: F, t19857: F, t25539: F, t375: F, t4783: F, t6285: F, t6317: F, t7125: F, t20020: F, t100275: F, t100289: F, t18904: F, t18913: F, t18937: F, t18942: F, t19861: F, t20040: F, t25495: F, t27526: F, t27527: F, t27531: F, t53321: F, t93752: F, t93801: F, t19907: F, t100327: F, t100329: F, t100332: F, t100334: F, t100336: F, t100342: F, t100343: F, t18909: F, t18926: F, t18930: F, t19912: F, t100121: F, t19626: F, t19641: F, t19731: F, t19819: F, t19873: F, t19944: F, t25517: F, t25569: F, t27536: F, t4788: F, t6268: F, t6293: F, t6327: F, t93548: F, t93821: F, t4845: F, t100324: F, t100359: F, t100363: F, t100365: F, t100370: F, t100398: F, t19645: F, t19917: F, t6289: F, t93731: F, t100135: F, t100168: F, t100255: F, t1047: F, t106877: F, t106913: F, t106929: F, t106943: F, t106968: F, t106990: F, t107012: F, t107035: F, t1671: F, t19702: F, t19800: F, t20096: F, t20101: F, t25512: F, t25526: F, t27450: F, t27489: F, t4803: F, t4808: F, t4825: F, t4869: F, t6263: F, t6302: F, t6308: F, t6312: F, t6323: F, t7122: F, t93567: F, t93696: F, t93764: F, t93783: F, t93796: F, t99983: F, t1096: F, t1695: F, t1983: F, t1984: F, t20152: F, t20188: F, t25591: F, t25692: F, t27419: F, t27550: F, t27576: F, t27616: F, t27679: F, t27699: F, t27703: F, t29747: F, t29875: F, t29887: F, t359: F, t4773: F, t4947: F, t5015: F, t5016: F, t6259: F, t7140: F, t7144: F, t7145: F, t7151: F, t7160: F, t7821: F, t94026: F, t988: F, t999: F, t99934: F, t1035: F, t29807: F, t29834: F, t7166: F, t1976: F, t6305: F, t3153: F, t6235: F, t100431: F, t100658: F, t1043: F, t1089: F, t1097: F, t1668: F, t1696: F, t19342: F, t19579: F, t20151: F, t25461: F, t25473: F, t25611: F, t27422: F, t27579: F, t27687: F, t29728: F, t29732: F, t29844: F, t6245: F, t7102: F, t7159: F, t7167: F, t7170: F, t93884: F, t93897: F, t99629: F, t99881: F, t11249: F, t4746: F, t7810: F, t7143: F, t1000: F, t100698: F, t100737: F, t1071: F, t19452: F, t1985: F, t1986: F, t25476: F, t27441: F, t27609: F, t27696: F, t29740: F, t29748: F, t29843: F, t4742: F, t6250: F, t7162: F, t93498: F, t93502: F, t93921: F, t94042: F, t99743: F, t99824: F, t106823: F, t106659: F, t106745: F, t106824: F, t1651: F, t20172: F, t25464: F, t25605: F, t25629: F, t27415: F, t27418: F, t27543: F, t27640: F, t27664: F, t27669: F, t27676: F, t29727: F, t29744: F, t29759: F, t29876: F, t3046: F, t4758: F, t4866: F, t4976: F, t4983: F, t4998: F, t6350: F, t7135: F, t7817: F, t7818: F, t7825: F, t93497: F, t93963: F, t93983: F, t99721: F) -> (F, F, F, F, F, F, F) {
        let (t107048, t107082) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2152::<F>(t19696, t7121, t20016, t25500, t19463, t1972, t100030, t100302, t100345, t1028, t1665, t19770, t19940, t19993, t19998, t25490, t25522, t27479, t4854, t6278, t6339, t7117, t93720, t93728);
        let t107103 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2153::<F>(t19976, t25580, t19900, t7111, t100030, t19718, t19831, t19973, t19982, t20070, t20075, t20091, t27493, t27498, t93658, t93667, t93745, t93750);
        let t107120 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2154::<F>(t1058, t29779, t100146, t100240, t100261, t100262, t100268, t100270, t100272, t1972, t19857, t25539, t375, t4783, t6285, t6317, t7125);
        let t107144 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2155::<F>(t20020, t7117, t100275, t100289, t18904, t18913, t18937, t18942, t19861, t20040, t25495, t27526, t27527, t27531, t53321, t6278, t93752, t93801);
        let t107159 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2156::<F>(t19907, t7111, t100327, t100329, t100332, t100334, t100336, t100342, t100343, t18909, t18926, t18930, t27526, t27527, t27531);
        let t107183 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2157::<F>(t19912, t7111, t100121, t100146, t19626, t19641, t19731, t19819, t19873, t19944, t25517, t25539, t25569, t27498, t27536, t4788, t6268, t6293, t6327, t93548, t93821);
        let t107197 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2158::<F>(t27479, t4845, t100324, t100359, t100363, t100365, t100370, t100398, t1665, t19645, t19917, t25517, t25539, t6289, t6339, t7111, t93731);
        let t107201 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2159::<F>(t100135, t100168, t100255, t1047, t106877, t106913, t106929, t106943, t106968, t106990, t107012, t107035, t107048, t107082, t107103, t107120, t107144, t107159, t107183, t107197, t1671, t19702, t19800, t20096, t20101, t25512, t25522, t25526, t25569, t27450, t27489, t4803, t4808, t4825, t4869, t6263, t6302, t6308, t6312, t6323, t7122, t93567, t93696, t93764, t93783, t93796, t99983);
        let t107206 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2160::<F>(t107201, t1096, t1695, t1983, t1984, t20152, t20188, t25591, t25692, t27419, t27550, t27576, t27616, t27679, t27699, t27703, t29747, t29875, t29887, t359, t4773, t4947, t5015, t5016, t6259, t7140, t7144, t7145, t7151, t7160, t7821, t94026, t988, t999, t99934);
        let (t107225, t107226, t107257) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2161::<F>(t1035, t29807, t29834, t7166, t1976, t6305, t3153, t6235, t100431, t100658, t1043, t1089, t1097, t1668, t1695, t1696, t19342, t19579, t20151, t25461, t25473, t25611, t27422, t27579, t27687, t29728, t29732, t29844, t6245, t7102, t7144, t7159, t7160, t7167, t7170, t93884, t93897, t99629, t99881);
        let (t107268, t107305) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2162::<F>(t107225, t11249, t4746, t7810, t29834, t7143, t1000, t100698, t100737, t1071, t1096, t19452, t1985, t1986, t25476, t27441, t27609, t27696, t29740, t29748, t29843, t29875, t4742, t6250, t7144, t7145, t7160, t7162, t93498, t93502, t93921, t94042, t988, t99743, t99824, t999);
        let t107354 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2163::<F>(t106823, t3153, t106659, t106745, t106824, t1089, t1096, t1651, t20172, t25464, t25605, t25611, t25629, t27415, t27418, t27543, t27640, t27664, t27669, t27676, t29727, t29744, t29759, t29876, t3046, t4758, t4866, t4976, t4983, t4998, t6350, t7135, t7140, t7145, t7151, t7159, t7160, t7817, t7818, t7825, t93497, t93963, t93983, t99721);
    (t107201, t107206, t107226, t107257, t107268, t107305, t107354)
}
