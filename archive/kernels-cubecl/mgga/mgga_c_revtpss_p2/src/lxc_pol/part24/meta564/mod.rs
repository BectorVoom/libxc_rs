//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta564 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1702;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1703;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1704;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1705;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1706;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1707;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1708;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1709;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1710;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1711;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1712;
use chunk11::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1713;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta564<F: Float>(t1011: F, t1042: F, t15935: F, t16095: F, t23857: F, t23886: F, t3092: F, t3117: F, t3127: F, t3162: F, t43044: F, t4578: F, t4834: F, t4919: F, t53762: F, t65859: F, t66022: F, t66029: F, t66141: F, t66218: F, t79290: F, t79309: F, t79315: F, t88124: F, t88925: F, t89084: F, t42078: F, t63453: F, t63459: F, t63464: F, t77499: F, t77559: F, t77561: F, t88085: F, t88089: F, t88093: F, t88097: F, t51978: F, t77505: F, t77507: F, t77509: F, t88104: F, t88108: F, t88114: F, t88118: F, t88122: F, t88126: F, t88130: F, t88134: F, t1025: F, t11703: F, t1469: F, t15618: F, t1651: F, t1665: F, t18936: F, t19773: F, t225: F, t23913: F, t24024: F, t24034: F, t366: F, t371: F, t372: F, t373: F, t375: F, t4858: F, t53878: F, t54118: F, t6268: F, t6278: F, t6312: F, t65654: F, t67528: F, t79428: F, t79439: F, t79474: F, t79864: F, t88675: F, t15707: F, t19697: F, t23635: F, t23643: F, t23823: F, t23834: F, t42745: F, t42920: F, t42921: F, t4879: F, t53704: F, t53707: F, t6302: F, t66547: F, t79546: F, t79548: F, t79553: F, t79564: F, t79575: F, t79580: F, t88695: F, t6258: F, t6299: F, t23820: F, t1045: F, t1063: F, t11859: F, t16067: F, t16199: F, t16208: F, t19501: F, t23908: F, t23921: F, t24009: F, t3115: F, t3155: F, t43174: F, t4837: F, t4872: F, t4892: F, t53800: F, t5819: F, t5825: F, t6244: F, t6266: F, t66721: F, t66763: F, t78496: F, t79638: F, t88732: F, t88804: F, t1012: F, t1015: F, t1675: F, t19968: F, t23485: F, t23859: F, t23911: F, t23976: F, t23980: F, t3091: F, t42518: F, t43223: F, t54687: F, t6323: F, t6327: F, t79559: F, t79742: F, t79744: F, t79758: F, t87126: F, t87145: F, t42508: F, t6271: F, t67015: F, t67186: F, t67195: F, t67206: F, t79811: F, t79818: F, t79874: F, t79881: F, t79892: F, t79938: F, t79944: F, t79946: F, t15906: F, t19450: F, t23898: F, t3205: F, t3236: F, t3253: F, t42731: F, t42977: F, t42978: F, t43155: F, t55247: F, t6339: F, t67473: F, t67502: F, t67575: F, t79957: F, t80038: F, t80113: F, t80277: F, t87107: F, t88727: F, t88763: F, t88800: F, t88849: F, t88898: F, t88944: F, t89009: F, t89046: F, t89094: F, t1076: F, t1079: F, t16312: F, t16313: F, t1647: F, t16600: F, t1695: F, t1696: F, t19351: F, t20178: F, t23599: F, t23617: F, t23620: F, t23621: F, t24044: F, t24048: F, t24061: F, t24177: F, t3058: F, t3269: F, t342: F, t385: F, t4747: F, t4778: F, t4935: F, t6235: F, t6345: F, t6351: F, t6392: F, t6393: F, t80983: F, t81052: F, t11121: F, t1652: F, t1680: F, t20175: F, t20191: F, t20204: F, t20211: F, t23603: F, t23959: F, t42060: F, t4752: F, t6245: F, t6251: F, t6259: F, t6350: F, t64687: F, t68022: F, t68144: F, t80173: F, t80810: F, t80901: F, t80921: F, t996: F, t6305: F, t6343: F, t378: F, t88714: F, t1678: F, t23640: F, t1082: F, t1087: F, t1089: F, t12047: F, t12052: F, t12149: F, t1668: F, t1689: F, t19446: F, t24042: F, t24108: F, t3204: F, t3299: F, t3304: F, t3317: F, t3318: F, t43154: F, t4954: F, t4975: F, t80243: F, t88815: F) -> (F, F, F, F, F, F, F, F, F) {
        let t89121 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1702::<F>(t1011, t1042, t15935, t16095, t23857, t23886, t3092, t3117, t3127, t3162, t43044, t4578, t4834, t4919, t53762, t65859, t66022, t66029, t66141, t66218, t79290, t79309, t79315, t88124, t88925, t89084);
        let (t89144, t89157) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1703::<F>(t42078, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t89158, t89180) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1704::<F>(t89144, t89157, t1025, t11703, t1469, t15618, t16095, t1651, t1665, t18936, t19773, t225, t23913, t24024, t24034, t366, t371, t372, t373, t375, t4858, t53878, t54118, t6268, t6278, t6312, t65654, t67528, t79428, t79439, t79474, t79864, t88675);
        let t89202 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1705::<F>(t1042, t15707, t19697, t23635, t23643, t23823, t23834, t42745, t42920, t42921, t4879, t53704, t53707, t6302, t66547, t79546, t79548, t79553, t79564, t79575, t79580, t88695);
        let (t89240, t89245, t89250) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1706::<F>(t6258, t6299, t1651, t23820, t1042, t1045, t1063, t11859, t15618, t16067, t16199, t16208, t19501, t23908, t23921, t24009, t3092, t3115, t3117, t3155, t43174, t4837, t4872, t4892, t53800, t5819, t5825, t6244, t6266, t66721, t66763, t78496, t79638, t88732, t88804);
        let t89283 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1707::<F>(t1011, t1012, t1015, t15707, t1675, t19968, t23485, t23859, t23911, t23976, t23980, t3091, t3092, t42518, t43223, t4834, t54687, t6323, t6327, t79559, t79742, t79744, t79758, t87126, t87145);
        let t89306 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1708::<F>(t1011, t1012, t11859, t3117, t3155, t42508, t6271, t6299, t67015, t67186, t67195, t67206, t79811, t79818, t79874, t79881, t79892, t79938, t79944, t79946, t87145);
        let (t89312, t89320, t89351) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1709::<F>(t6258, t6244, t1011, t1012, t1042, t15906, t19450, t23898, t3092, t3117, t3205, t3236, t3253, t371, t372, t373, t42731, t42977, t42978, t43155, t55247, t6339, t67473, t67502, t67575, t78496, t79957, t80038, t80113, t80277, t87107, t87145, t88695);
        let t89355 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1710::<F>(t88727, t88763, t88800, t88849, t88898, t88944, t89009, t89046, t89094, t89121, t89180, t89202, t89250, t89283, t89306, t89351);
        let t89397 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1711::<F>(t1076, t1079, t16312, t16313, t1647, t16600, t1695, t1696, t19351, t20178, t225, t23599, t23617, t23620, t23621, t24044, t24048, t24061, t24177, t3058, t3269, t342, t385, t4747, t4778, t4935, t6235, t6244, t6345, t6351, t6392, t6393, t80983, t81052, t89355);
        let t89437 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1712::<F>(t1076, t11121, t1652, t1680, t20175, t20178, t20191, t20204, t20211, t23603, t23959, t3058, t42060, t4752, t6245, t6251, t6259, t6350, t6351, t6392, t64687, t68022, t68144, t80173, t80810, t80901, t80921, t89312, t89320, t996);
        let (t89490, t89503, t89507) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1713::<F>(t6305, t6343, t378, t88714, t1678, t23640, t1082, t1087, t1089, t12047, t12052, t12149, t1668, t1689, t19446, t24042, t24108, t3204, t3299, t3304, t3317, t3318, t43154, t4954, t4975, t6258, t80243, t88815, t89312, t89320);
    (t89158, t89240, t89245, t89355, t89397, t89437, t89490, t89503, t89507)
}
