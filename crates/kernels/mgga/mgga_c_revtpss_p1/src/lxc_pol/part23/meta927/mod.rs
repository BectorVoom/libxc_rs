//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta927 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3009;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3010;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3011;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3012;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3013;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3014;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3015;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3016;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3017;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3018;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3019;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta927<F: Float>(t1011: F, t140: F, t23868: F, t41361: F, t42078: F, t51978: F, t53243: F, t63276: F, t63278: F, t77499: F, t77503: F, t77505: F, t77507: F, t77509: F, t77515: F, t77518: F, t77521: F, t77527: F, t77531: F, t77535: F, t77539: F, t77543: F, t77547: F, t53252: F, t53253: F, t63338: F, t63340: F, t63342: F, t63361: F, t63371: F, t63447: F, t63453: F, t63459: F, t63464: F, t77559: F, t77561: F, t77566: F, t77570: F, t77575: F, t77581: F, t77586: F, t77590: F, t77594: F, t1025: F, t20039: F, t23869: F, t23874: F, t24024: F, t24034: F, t3224: F, t3241: F, t371: F, t372: F, t373: F, t42262: F, t43069: F, t4915: F, t4919: F, t66689: F, t67327: F, t67329: F, t67353: F, t67355: F, t67358: F, t67426: F, t77525: F, t77529: F, t77533: F, t77537: F, t77588: F, t77592: F, t11922: F, t23930: F, t4892: F, t1469: F, t18281: F, t4186: F, t5825: F, t1042: F, t1063: F, t11994: F, t16089: F, t16199: F, t23844: F, t23848: F, t3092: F, t3127: F, t3188: F, t4757: F, t4801: F, t4806: F, t55247: F, t55272: F, t55280: F, t6096: F, t67435: F, t67473: F, t67493: F, t67499: F, t67521: F, t67526: F, t78570: F, t78765: F, t11710: F, t23903: F, t4899: F, t11250: F, t11933: F, t15689: F, t15691: F, t15700: F, t16049: F, t16222: F, t16226: F, t19622: F, t19626: F, t19741: F, t19996: F, t23904: F, t23939: F, t23999: F, t3091: F, t3095: F, t3162: F, t43066: F, t4579: F, t54500: F, t54658: F, t54672: F, t54801: F, t55294: F, t66187: F, t67560: F, t67568: F, t67571: F, t67575: F, t79159: F, t79395: F, t79463: F, t79467: F, t79770: F, t78545: F, t78601: F, t78662: F, t78745: F, t78807: F, t78857: F, t78909: F, t78954: F, t79006: F, t79049: F, t79105: F, t79151: F, t79206: F, t79255: F, t79287: F, t79331: F, t79407: F, t79456: F, t79514: F, t79550: F, t79588: F, t79627: F, t79665: F, t79723: F, t79768: F, t79822: F, t79870: F, t79907: F, t79951: F, t11187: F, t11224: F, t16333: F, t16362: F, t1647: F, t16603: F, t16604: F, t1696: F, t19342: F, t19384: F, t19385: F, t19428: F, t20168: F, t20171: F, t20211: F, t20215: F, t225: F, t23583: F, t23607: F, t24048: F, t24061: F, t3047: F, t3052: F, t3269: F, t342: F, t385: F, t4743: F, t4747: F, t4764: F, t4772: F, t4773: F, t4778: F, t6345: F, t6350: F, t6351: F, t6393: F, t64605: F, t64639: F, t995: F, t1678: F, t19462: F, t1000: F, t16312: F, t16313: F, t16340: F, t16371: F, t16374: F, t16597: F, t19351: F, t19396: F, t19424: F, t20188: F, t20218: F, t23599: F, t23603: F, t24068: F, t24178: F, t3058: F, t3264: F, t42052: F, t4940: F, t5016: F, t53160: F, t6251: F, t64614: F, t68018: F, t79480: F, t996: F, t1086: F, t23959: F, t11249: F, t24007: F, t23997: F, t3153: F, t1083: F, t1087: F, t1089: F, t1090: F, t12127: F, t1668: F, t1685: F, t19447: F, t19452: F, t19477: F, t19488: F, t19503: F, t20112: F, t24167: F, t3223: F, t378: F, t43341: F, t4954: F, t4998: F, t55747: F, t55988: F, t64907: F, t78721: F, t79863: F, t3154: F, t6299: F, t12050: F, t12122: F, t16432: F, t16566: F, t16584: F, t19414: F, t19491: F, t19534: F, t19548: F, t19584: F, t19594: F, t19597: F, t24116: F, t24162: F, t3204: F, t3278: F, t380: F, t43520: F, t43524: F, t5004: F, t55732: F, t55958: F, t56049: F, t6375: F, t65144: F, t79703: F, t989: F, t999: F, t23992: F, t23837: F, t1071: F, t23640: F, t12078: F, t12079: F, t12116: F, t12149: F, t19483: F, t19579: F, t19593: F, t24079: F, t24112: F, t43438: F, t43456: F, t43574: F, t4976: F, t4983: F, t55887: F, t6389: F, t73: F, t357: F, t11631: F, t1043: F, t11940: F, t12047: F, t12052: F, t16502: F, t16552: F, t16559: F, t16560: F, t19450: F, t19456: F, t19502: F, t20123: F, t20139: F, t20146: F, t4866: F, t5012: F, t55499: F, t55646: F, t6235: F, t6365: F, t78496: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t79957, t80008) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3009::<F>(t1011, t140, t23868, t41361, t42078, t51978, t53243, t63276, t63278, t77499, t77503, t77505, t77507, t77509, t77515, t77518, t77521, t77527, t77531, t77535, t77539, t77543, t77547);
        let t80027 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3010::<F>(t53252, t53253, t63338, t63340, t63342, t63361, t63371, t63447, t63453, t63459, t63464, t77559, t77561, t77566, t77570, t77575, t77581, t77586, t77590, t77594);
        let (t80028, t80034) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3011::<F>(t80008, t80027, t1011, t1025, t20039, t23869, t23874, t24024, t24034, t3224, t3241, t371, t372, t373, t42262, t43069, t4915, t4919, t66689, t67327, t67329, t67353, t67355, t67358, t67426, t77525, t77529, t77533, t77537, t77588, t77592, t79957);
        let (t80045, t80050, t80081) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3012::<F>(t11922, t23930, t4892, t1469, t18281, t4186, t5825, t1042, t1063, t11994, t16089, t16199, t23844, t23848, t3092, t3127, t3188, t4757, t4801, t4806, t55247, t55272, t55280, t6096, t67435, t67473, t67493, t67499, t67521, t67526, t78570, t78765);
        let t80127 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3013::<F>(t11710, t23903, t4899, t11250, t11933, t15689, t15691, t15700, t16049, t16222, t16226, t19622, t19626, t19741, t19996, t23904, t23939, t23999, t3091, t3092, t3095, t3162, t43066, t4579, t54500, t54658, t54672, t54801, t55294, t66187, t67560, t67568, t67571, t67575, t79159, t79395, t79463, t79467, t79770);
        let t80132 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3014::<F>(t78545, t78601, t78662, t78745, t78807, t78857, t78909, t78954, t79006, t79049, t79105, t79151, t79206, t79255, t79287, t79331, t79407, t79456, t79514, t79550, t79588, t79627, t79665, t79723, t79768, t79822, t79870, t79907, t79951, t80034, t80081, t80127);
        let t80166 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3015::<F>(t11187, t11224, t16333, t16362, t1647, t16603, t16604, t1696, t19342, t19384, t19385, t19428, t20168, t20171, t20211, t20215, t225, t23583, t23607, t24048, t24061, t3047, t3052, t3269, t342, t385, t4743, t4747, t4764, t4772, t4773, t4778, t6345, t6350, t6351, t6393, t64605, t64639, t80132, t995);
        let t80211 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3016::<F>(t1678, t19462, t1000, t16312, t16313, t16340, t16362, t16371, t16374, t16597, t16603, t19351, t19384, t19396, t19424, t20188, t20218, t23599, t23603, t24068, t24178, t3047, t3052, t3058, t3264, t42052, t4778, t4940, t5016, t53160, t6251, t6351, t6393, t64614, t68018, t79480, t996);
        let (t80248, t80264, t80274) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3017::<F>(t1086, t23959, t11249, t24007, t23997, t3153, t1083, t1087, t1089, t1090, t12127, t1668, t1678, t1685, t19447, t19452, t19477, t19488, t19503, t20112, t24167, t3223, t378, t43341, t4954, t4998, t55747, t55988, t64907, t78721, t79863);
        let (t80277, t80310) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3018::<F>(t3154, t6299, t12050, t12122, t16432, t16566, t16584, t19414, t19491, t19534, t19548, t19584, t19594, t19597, t24116, t24162, t3204, t3278, t342, t380, t43520, t43524, t4954, t5004, t55732, t55958, t56049, t6375, t65144, t79703, t80132, t80248, t989, t999);
        let (t80312, t80341, t80349) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3019::<F>(t24007, t3153, t23992, t23837, t1071, t23640, t12078, t12079, t12116, t12122, t12127, t12149, t19447, t19483, t19579, t19593, t23997, t24079, t24112, t43438, t43456, t43574, t4743, t4976, t4983, t4998, t55887, t6389, t73);
        let t80391 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3020::<F>(t12050, t357, t11631, t6299, t1043, t11940, t12047, t12052, t16502, t16552, t16559, t16560, t16566, t19450, t19456, t19502, t20123, t20139, t20146, t43341, t43438, t4866, t4954, t5004, t5012, t55499, t55646, t55887, t6235, t6365, t78496, t80277, t80312, t80341, t999);
    (t80028, t80045, t80050, t80166, t80211, t80264, t80274, t80310, t80341, t80349, t80391)
}
