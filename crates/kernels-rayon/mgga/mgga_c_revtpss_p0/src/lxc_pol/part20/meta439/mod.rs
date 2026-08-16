//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1663;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1665;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1667;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1668;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1669;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta439(t1198: f64, t45319: f64, t12222: f64, t3531: f64, t1196: f64, t12234: f64, t12548: f64, t45282: f64, t45296: f64, t45298: f64, t45302: f64, t45306: f64, t45310: f64, t45312: f64, t45316: f64, t45318: f64, t44982: f64, t45016: f64, t45053: f64, t13062: f64, t13064: f64, t3172: f64, t1012: f64, t1042: f64, t1222: f64, t1225: f64, t1247: f64, t1250: f64, t12922: f64, t12956: f64, t13079: f64, t247: f64, t3368: f64, t3372: f64, t3611: f64, t3719: f64, t3720: f64, t39443: f64, t39457: f64, t44552: f64, t44944: f64, t44949: f64, t44952: f64, t44959: f64, t44965: f64, t44972: f64, t44974: f64, t44980: f64, t482: f64, t5384: f64, t13075: f64, t1209: f64, t13126: f64, t17708: f64, t127: f64, t12988: f64, t12989: f64, t371: f64, t1203: f64, t12626: f64, t225: f64, t480: f64, t12967: f64, t12995: f64, t3584: f64, t1122: f64, t12621: f64, t1263: f64, t12832: f64, t12862: f64, t12872: f64, t12876: f64, t12953: f64, t12991: f64, t17426: f64, t17429: f64, t17475: f64, t17654: f64, t17657: f64, t17703: f64, t17747: f64, t17753: f64, t17784: f64, t3671: f64, t3711: f64, t372: f64, t43835: f64, t44501: f64, t44585: f64, t44808: f64, t5352: f64, t44185: f64, t44239: f64, t44282: f64, t44353: f64, t44417: f64, t44479: f64, t44529: f64, t44595: f64, t44657: f64, t44706: f64, t44758: f64, t44812: f64, t44894: f64, t44942: f64, t12627: f64, t1269: f64, t3566: f64, t3727: f64, t12640: f64, t44842: f64, t487: f64, t1204: f64, t1210: f64, t1211: f64, t1214: f64, t12603: f64, t12630: f64, t12651: f64, t12654: f64, t12658: f64, t12673: f64, t12690: f64, t12696: f64, t1271: f64, t13170: f64, t13174: f64, t13182: f64, t13183: f64, t3552: f64, t3556: f64, t3569: f64, t3572: f64, t3585: f64, t3729: f64, t3732: f64, t3739: f64, t3791: f64, t44321: f64, t44845: f64, t460: f64, t494: f64, t495: f64, t44420: f64, t12600: f64, t12622: f64, t12633: f64, t12695: f64, t1276: f64, t1277: f64, t1295: f64, t13177: f64, t13184: f64, t17973: f64, t17986: f64, t3567: f64, t3568: f64, t3575: f64, t3576: f64, t3736: f64, t3790: f64, t44878: f64, t44831: f64, t12657: f64, t1215: f64, t12607: f64, t12628: f64, t12629: f64, t12641: f64, t12647: f64, t12666: f64, t1274: f64, t1294: f64, t13166: f64, t3561: f64, t3737: f64, t3738: f64, t44778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45321, t45323, t45326, t45327) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1663(t1198, t45319, t12222, t3531, t1196, t12234, t12548, t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318);
        let (t45329, t45348) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664(t44982, t45016, t45053, t45327, t13062, t13064, t3172, t1012, t1042, t1222, t1225, t1247, t1250, t12922, t12956, t13079, t247, t3368, t3372, t3611, t3719, t3720, t39443, t39457, t44552, t44944, t44949, t44952, t44959, t44965, t44972, t44974, t44980, t482, t5384);
        let (t45352, t45371, t45382, t45384) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1665(t1247, t13075, t3172, t1209, t13126, t17708, t127, t12988, t12989, t371, t1203, t12626);
        let (t45385, t45391, t45402) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666(t225, t45384, t480, t12967, t12995, t3584, t1042, t1122, t1222, t12621, t1263, t12832, t12862, t12872, t12876, t12953, t12956, t12991, t17426, t17429, t17475, t17654, t17657, t17703, t17747, t17753, t17784, t3671, t371, t3711, t372, t3720, t43835, t44501, t44585, t44808, t45352, t45371, t45382, t482, t5352);
        let t45406 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1667(t44185, t44239, t44282, t44353, t44417, t44479, t44529, t44595, t44657, t44706, t44758, t44812, t44894, t44942, t45348, t45402);
        let t45448 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1668(t12627, t1269, t3566, t3727, t12640, t44842, t487, t1204, t1210, t1211, t1214, t12603, t12630, t12651, t12654, t12658, t12673, t12690, t12696, t1271, t13170, t13174, t13182, t13183, t225, t3552, t3556, t3569, t3572, t3585, t3729, t3732, t3739, t3791, t44321, t44845, t45406, t460, t494, t495);
        let t45494 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1669(t45384, t487, t1269, t3552, t44420, t12690, t1210, t1211, t1214, t12600, t12603, t12622, t12630, t12633, t12651, t12654, t12673, t12695, t1276, t1277, t1295, t13174, t13177, t13184, t17973, t17986, t3556, t3567, t3568, t3569, t3572, t3575, t3576, t3584, t3732, t3736, t3739, t3790, t3791, t44878);
        let t45544 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1670(t44831, t487, t12657, t1269, t1204, t3727, t1210, t1211, t1215, t12600, t12607, t12621, t12628, t12629, t12633, t12641, t12647, t12666, t1274, t1277, t1294, t1295, t13166, t13182, t13184, t3561, t3572, t3576, t3584, t3585, t3732, t3737, t3738, t3790, t44778);
    (t45321, t45323, t45326, t45329, t45385, t45391, t45406, t45448, t45494, t45544)
}
