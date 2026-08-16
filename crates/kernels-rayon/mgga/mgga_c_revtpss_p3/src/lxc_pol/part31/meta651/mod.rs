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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta651(t19696: f64, t7121: f64, t20016: f64, t25500: f64, t19463: f64, t1972: f64, t100030: f64, t100302: f64, t100345: f64, t1028: f64, t1665: f64, t19770: f64, t19940: f64, t19993: f64, t19998: f64, t25490: f64, t25522: f64, t27479: f64, t4854: f64, t6278: f64, t6339: f64, t7117: f64, t93720: f64, t93728: f64, t19976: f64, t25580: f64, t19900: f64, t7111: f64, t19718: f64, t19831: f64, t19973: f64, t19982: f64, t20070: f64, t20075: f64, t20091: f64, t27493: f64, t27498: f64, t93658: f64, t93667: f64, t93745: f64, t93750: f64, t1058: f64, t29779: f64, t100146: f64, t100240: f64, t100261: f64, t100262: f64, t100268: f64, t100270: f64, t100272: f64, t19857: f64, t25539: f64, t375: f64, t4783: f64, t6285: f64, t6317: f64, t7125: f64, t20020: f64, t100275: f64, t100289: f64, t18904: f64, t18913: f64, t18937: f64, t18942: f64, t19861: f64, t20040: f64, t25495: f64, t27526: f64, t27527: f64, t27531: f64, t53321: f64, t93752: f64, t93801: f64, t19907: f64, t100327: f64, t100329: f64, t100332: f64, t100334: f64, t100336: f64, t100342: f64, t100343: f64, t18909: f64, t18926: f64, t18930: f64, t19912: f64, t100121: f64, t19626: f64, t19641: f64, t19731: f64, t19819: f64, t19873: f64, t19944: f64, t25517: f64, t25569: f64, t27536: f64, t4788: f64, t6268: f64, t6293: f64, t6327: f64, t93548: f64, t93821: f64, t4845: f64, t100324: f64, t100359: f64, t100363: f64, t100365: f64, t100370: f64, t100398: f64, t19645: f64, t19917: f64, t6289: f64, t93731: f64, t100135: f64, t100168: f64, t100255: f64, t1047: f64, t106877: f64, t106913: f64, t106929: f64, t106943: f64, t106968: f64, t106990: f64, t107012: f64, t107035: f64, t1671: f64, t19702: f64, t19800: f64, t20096: f64, t20101: f64, t25512: f64, t25526: f64, t27450: f64, t27489: f64, t4803: f64, t4808: f64, t4825: f64, t4869: f64, t6263: f64, t6302: f64, t6308: f64, t6312: f64, t6323: f64, t7122: f64, t93567: f64, t93696: f64, t93764: f64, t93783: f64, t93796: f64, t99983: f64, t1096: f64, t1695: f64, t1983: f64, t1984: f64, t20152: f64, t20188: f64, t25591: f64, t25692: f64, t27419: f64, t27550: f64, t27576: f64, t27616: f64, t27679: f64, t27699: f64, t27703: f64, t29747: f64, t29875: f64, t29887: f64, t359: f64, t4773: f64, t4947: f64, t5015: f64, t5016: f64, t6259: f64, t7140: f64, t7144: f64, t7145: f64, t7151: f64, t7160: f64, t7821: f64, t94026: f64, t988: f64, t999: f64, t99934: f64, t1035: f64, t29807: f64, t29834: f64, t7166: f64, t1976: f64, t6305: f64, t3153: f64, t6235: f64, t100431: f64, t100658: f64, t1043: f64, t1089: f64, t1097: f64, t1668: f64, t1696: f64, t19342: f64, t19579: f64, t20151: f64, t25461: f64, t25473: f64, t25611: f64, t27422: f64, t27579: f64, t27687: f64, t29728: f64, t29732: f64, t29844: f64, t6245: f64, t7102: f64, t7159: f64, t7167: f64, t7170: f64, t93884: f64, t93897: f64, t99629: f64, t99881: f64, t11249: f64, t4746: f64, t7810: f64, t7143: f64, t1000: f64, t100698: f64, t100737: f64, t1071: f64, t19452: f64, t1985: f64, t1986: f64, t25476: f64, t27441: f64, t27609: f64, t27696: f64, t29740: f64, t29748: f64, t29843: f64, t4742: f64, t6250: f64, t7162: f64, t93498: f64, t93502: f64, t93921: f64, t94042: f64, t99743: f64, t99824: f64, t106823: f64, t106659: f64, t106745: f64, t106824: f64, t1651: f64, t20172: f64, t25464: f64, t25605: f64, t25629: f64, t27415: f64, t27418: f64, t27543: f64, t27640: f64, t27664: f64, t27669: f64, t27676: f64, t29727: f64, t29744: f64, t29759: f64, t29876: f64, t3046: f64, t4758: f64, t4866: f64, t4976: f64, t4983: f64, t4998: f64, t6350: f64, t7135: f64, t7817: f64, t7818: f64, t7825: f64, t93497: f64, t93963: f64, t93983: f64, t99721: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t107048, t107082) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2152(t19696, t7121, t20016, t25500, t19463, t1972, t100030, t100302, t100345, t1028, t1665, t19770, t19940, t19993, t19998, t25490, t25522, t27479, t4854, t6278, t6339, t7117, t93720, t93728);
        let t107103 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2153(t19976, t25580, t19900, t7111, t100030, t19718, t19831, t19973, t19982, t20070, t20075, t20091, t27493, t27498, t93658, t93667, t93745, t93750);
        let t107120 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2154(t1058, t29779, t100146, t100240, t100261, t100262, t100268, t100270, t100272, t1972, t19857, t25539, t375, t4783, t6285, t6317, t7125);
        let t107144 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2155(t20020, t7117, t100275, t100289, t18904, t18913, t18937, t18942, t19861, t20040, t25495, t27526, t27527, t27531, t53321, t6278, t93752, t93801);
        let t107159 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2156(t19907, t7111, t100327, t100329, t100332, t100334, t100336, t100342, t100343, t18909, t18926, t18930, t27526, t27527, t27531);
        let t107183 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2157(t19912, t7111, t100121, t100146, t19626, t19641, t19731, t19819, t19873, t19944, t25517, t25539, t25569, t27498, t27536, t4788, t6268, t6293, t6327, t93548, t93821);
        let t107197 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2158(t27479, t4845, t100324, t100359, t100363, t100365, t100370, t100398, t1665, t19645, t19917, t25517, t25539, t6289, t6339, t7111, t93731);
        let t107201 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2159(t100135, t100168, t100255, t1047, t106877, t106913, t106929, t106943, t106968, t106990, t107012, t107035, t107048, t107082, t107103, t107120, t107144, t107159, t107183, t107197, t1671, t19702, t19800, t20096, t20101, t25512, t25522, t25526, t25569, t27450, t27489, t4803, t4808, t4825, t4869, t6263, t6302, t6308, t6312, t6323, t7122, t93567, t93696, t93764, t93783, t93796, t99983);
        let t107206 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2160(t107201, t1096, t1695, t1983, t1984, t20152, t20188, t25591, t25692, t27419, t27550, t27576, t27616, t27679, t27699, t27703, t29747, t29875, t29887, t359, t4773, t4947, t5015, t5016, t6259, t7140, t7144, t7145, t7151, t7160, t7821, t94026, t988, t999, t99934);
        let (t107225, t107226, t107257) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2161(t1035, t29807, t29834, t7166, t1976, t6305, t3153, t6235, t100431, t100658, t1043, t1089, t1097, t1668, t1695, t1696, t19342, t19579, t20151, t25461, t25473, t25611, t27422, t27579, t27687, t29728, t29732, t29844, t6245, t7102, t7144, t7159, t7160, t7167, t7170, t93884, t93897, t99629, t99881);
        let (t107268, t107305) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2162(t107225, t11249, t4746, t7810, t29834, t7143, t1000, t100698, t100737, t1071, t1096, t19452, t1985, t1986, t25476, t27441, t27609, t27696, t29740, t29748, t29843, t29875, t4742, t6250, t7144, t7145, t7160, t7162, t93498, t93502, t93921, t94042, t988, t99743, t99824, t999);
        let t107354 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2163(t106823, t3153, t106659, t106745, t106824, t1089, t1096, t1651, t20172, t25464, t25605, t25611, t25629, t27415, t27418, t27543, t27640, t27664, t27669, t27676, t29727, t29744, t29759, t29876, t3046, t4758, t4866, t4976, t4983, t4998, t6350, t7135, t7140, t7145, t7151, t7159, t7160, t7817, t7818, t7825, t93497, t93963, t93983, t99721);
    (t107201, t107206, t107226, t107257, t107268, t107305, t107354)
}
