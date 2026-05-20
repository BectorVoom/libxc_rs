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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1663;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1665;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1667;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1668;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1669;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta439<F: Float>(t1198: F, t45319: F, t12222: F, t3531: F, t1196: F, t12234: F, t12548: F, t45282: F, t45296: F, t45298: F, t45302: F, t45306: F, t45310: F, t45312: F, t45316: F, t45318: F, t44982: F, t45016: F, t45053: F, t13062: F, t13064: F, t3172: F, t1012: F, t1042: F, t1222: F, t1225: F, t1247: F, t1250: F, t12922: F, t12956: F, t13079: F, t247: F, t3368: F, t3372: F, t3611: F, t3719: F, t3720: F, t39443: F, t39457: F, t44552: F, t44944: F, t44949: F, t44952: F, t44959: F, t44965: F, t44972: F, t44974: F, t44980: F, t482: F, t5384: F, t13075: F, t1209: F, t13126: F, t17708: F, t127: F, t12988: F, t12989: F, t371: F, t1203: F, t12626: F, t225: F, t480: F, t12967: F, t12995: F, t3584: F, t1122: F, t12621: F, t1263: F, t12832: F, t12862: F, t12872: F, t12876: F, t12953: F, t12991: F, t17426: F, t17429: F, t17475: F, t17654: F, t17657: F, t17703: F, t17747: F, t17753: F, t17784: F, t3671: F, t3711: F, t372: F, t43835: F, t44501: F, t44585: F, t44808: F, t5352: F, t44185: F, t44239: F, t44282: F, t44353: F, t44417: F, t44479: F, t44529: F, t44595: F, t44657: F, t44706: F, t44758: F, t44812: F, t44894: F, t44942: F, t12627: F, t1269: F, t3566: F, t3727: F, t12640: F, t44842: F, t487: F, t1204: F, t1210: F, t1211: F, t1214: F, t12603: F, t12630: F, t12651: F, t12654: F, t12658: F, t12673: F, t12690: F, t12696: F, t1271: F, t13170: F, t13174: F, t13182: F, t13183: F, t3552: F, t3556: F, t3569: F, t3572: F, t3585: F, t3729: F, t3732: F, t3739: F, t3791: F, t44321: F, t44845: F, t460: F, t494: F, t495: F, t44420: F, t12600: F, t12622: F, t12633: F, t12695: F, t1276: F, t1277: F, t1295: F, t13177: F, t13184: F, t17973: F, t17986: F, t3567: F, t3568: F, t3575: F, t3576: F, t3736: F, t3790: F, t44878: F, t44831: F, t12657: F, t1215: F, t12607: F, t12628: F, t12629: F, t12641: F, t12647: F, t12666: F, t1274: F, t1294: F, t13166: F, t3561: F, t3737: F, t3738: F, t44778: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t45321, t45323, t45326, t45327) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1663::<F>(t1198, t45319, t12222, t3531, t1196, t12234, t12548, t45282, t45296, t45298, t45302, t45306, t45310, t45312, t45316, t45318);
        let (t45329, t45348) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1664::<F>(t44982, t45016, t45053, t45327, t13062, t13064, t3172, t1012, t1042, t1222, t1225, t1247, t1250, t12922, t12956, t13079, t247, t3368, t3372, t3611, t3719, t3720, t39443, t39457, t44552, t44944, t44949, t44952, t44959, t44965, t44972, t44974, t44980, t482, t5384);
        let (t45352, t45371, t45382, t45384) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1665::<F>(t1247, t13075, t3172, t1209, t13126, t17708, t127, t12988, t12989, t371, t1203, t12626);
        let (t45385, t45391, t45402) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666::<F>(t225, t45384, t480, t12967, t12995, t3584, t1042, t1122, t1222, t12621, t1263, t12832, t12862, t12872, t12876, t12953, t12956, t12991, t17426, t17429, t17475, t17654, t17657, t17703, t17747, t17753, t17784, t3671, t371, t3711, t372, t3720, t43835, t44501, t44585, t44808, t45352, t45371, t45382, t482, t5352);
        let t45406 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1667::<F>(t44185, t44239, t44282, t44353, t44417, t44479, t44529, t44595, t44657, t44706, t44758, t44812, t44894, t44942, t45348, t45402);
        let t45448 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1668::<F>(t12627, t1269, t3566, t3727, t12640, t44842, t487, t1204, t1210, t1211, t1214, t12603, t12630, t12651, t12654, t12658, t12673, t12690, t12696, t1271, t13170, t13174, t13182, t13183, t225, t3552, t3556, t3569, t3572, t3585, t3729, t3732, t3739, t3791, t44321, t44845, t45406, t460, t494, t495);
        let t45494 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1669::<F>(t45384, t487, t1269, t3552, t44420, t12690, t1210, t1211, t1214, t12600, t12603, t12622, t12630, t12633, t12651, t12654, t12673, t12695, t1276, t1277, t1295, t13174, t13177, t13184, t17973, t17986, t3556, t3567, t3568, t3569, t3572, t3575, t3576, t3584, t3732, t3736, t3739, t3790, t3791, t44878);
        let t45544 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1670::<F>(t44831, t487, t12657, t1269, t1204, t3727, t1210, t1211, t1215, t12600, t12607, t12621, t12628, t12629, t12633, t12641, t12647, t12666, t1274, t1277, t1294, t1295, t13166, t13182, t13184, t3561, t3572, t3576, t3584, t3585, t3732, t3737, t3738, t3790, t44778);
    (t45321, t45323, t45326, t45329, t45385, t45391, t45406, t45448, t45494, t45544)
}
