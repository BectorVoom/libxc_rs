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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta564(t1011: f64, t1042: f64, t15935: f64, t16095: f64, t23857: f64, t23886: f64, t3092: f64, t3117: f64, t3127: f64, t3162: f64, t43044: f64, t4578: f64, t4834: f64, t4919: f64, t53762: f64, t65859: f64, t66022: f64, t66029: f64, t66141: f64, t66218: f64, t79290: f64, t79309: f64, t79315: f64, t88124: f64, t88925: f64, t89084: f64, t42078: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64, t51978: f64, t77505: f64, t77507: f64, t77509: f64, t88104: f64, t88108: f64, t88114: f64, t88118: f64, t88122: f64, t88126: f64, t88130: f64, t88134: f64, t1025: f64, t11703: f64, t1469: f64, t15618: f64, t1651: f64, t1665: f64, t18936: f64, t19773: f64, t225: f64, t23913: f64, t24024: f64, t24034: f64, t366: f64, t371: f64, t372: f64, t373: f64, t375: f64, t4858: f64, t53878: f64, t54118: f64, t6268: f64, t6278: f64, t6312: f64, t65654: f64, t67528: f64, t79428: f64, t79439: f64, t79474: f64, t79864: f64, t88675: f64, t15707: f64, t19697: f64, t23635: f64, t23643: f64, t23823: f64, t23834: f64, t42745: f64, t42920: f64, t42921: f64, t4879: f64, t53704: f64, t53707: f64, t6302: f64, t66547: f64, t79546: f64, t79548: f64, t79553: f64, t79564: f64, t79575: f64, t79580: f64, t88695: f64, t6258: f64, t6299: f64, t23820: f64, t1045: f64, t1063: f64, t11859: f64, t16067: f64, t16199: f64, t16208: f64, t19501: f64, t23908: f64, t23921: f64, t24009: f64, t3115: f64, t3155: f64, t43174: f64, t4837: f64, t4872: f64, t4892: f64, t53800: f64, t5819: f64, t5825: f64, t6244: f64, t6266: f64, t66721: f64, t66763: f64, t78496: f64, t79638: f64, t88732: f64, t88804: f64, t1012: f64, t1015: f64, t1675: f64, t19968: f64, t23485: f64, t23859: f64, t23911: f64, t23976: f64, t23980: f64, t3091: f64, t42518: f64, t43223: f64, t54687: f64, t6323: f64, t6327: f64, t79559: f64, t79742: f64, t79744: f64, t79758: f64, t87126: f64, t87145: f64, t42508: f64, t6271: f64, t67015: f64, t67186: f64, t67195: f64, t67206: f64, t79811: f64, t79818: f64, t79874: f64, t79881: f64, t79892: f64, t79938: f64, t79944: f64, t79946: f64, t15906: f64, t19450: f64, t23898: f64, t3205: f64, t3236: f64, t3253: f64, t42731: f64, t42977: f64, t42978: f64, t43155: f64, t55247: f64, t6339: f64, t67473: f64, t67502: f64, t67575: f64, t79957: f64, t80038: f64, t80113: f64, t80277: f64, t87107: f64, t88727: f64, t88763: f64, t88800: f64, t88849: f64, t88898: f64, t88944: f64, t89009: f64, t89046: f64, t89094: f64, t1076: f64, t1079: f64, t16312: f64, t16313: f64, t1647: f64, t16600: f64, t1695: f64, t1696: f64, t19351: f64, t20178: f64, t23599: f64, t23617: f64, t23620: f64, t23621: f64, t24044: f64, t24048: f64, t24061: f64, t24177: f64, t3058: f64, t3269: f64, t342: f64, t385: f64, t4747: f64, t4778: f64, t4935: f64, t6235: f64, t6345: f64, t6351: f64, t6392: f64, t6393: f64, t80983: f64, t81052: f64, t11121: f64, t1652: f64, t1680: f64, t20175: f64, t20191: f64, t20204: f64, t20211: f64, t23603: f64, t23959: f64, t42060: f64, t4752: f64, t6245: f64, t6251: f64, t6259: f64, t6350: f64, t64687: f64, t68022: f64, t68144: f64, t80173: f64, t80810: f64, t80901: f64, t80921: f64, t996: f64, t6305: f64, t6343: f64, t378: f64, t88714: f64, t1678: f64, t23640: f64, t1082: f64, t1087: f64, t1089: f64, t12047: f64, t12052: f64, t12149: f64, t1668: f64, t1689: f64, t19446: f64, t24042: f64, t24108: f64, t3204: f64, t3299: f64, t3304: f64, t3317: f64, t3318: f64, t43154: f64, t4954: f64, t4975: f64, t80243: f64, t88815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t89121 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1702(t1011, t1042, t15935, t16095, t23857, t23886, t3092, t3117, t3127, t3162, t43044, t4578, t4834, t4919, t53762, t65859, t66022, t66029, t66141, t66218, t79290, t79309, t79315, t88124, t88925, t89084);
        let (t89144, t89157) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1703(t42078, t63453, t63459, t63464, t77499, t77559, t77561, t88085, t88089, t88093, t88097, t51978, t77505, t77507, t77509, t88104, t88108, t88114, t88118, t88122, t88126, t88130, t88134);
        let (t89158, t89180) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1704(t89144, t89157, t1025, t11703, t1469, t15618, t16095, t1651, t1665, t18936, t19773, t225, t23913, t24024, t24034, t366, t371, t372, t373, t375, t4858, t53878, t54118, t6268, t6278, t6312, t65654, t67528, t79428, t79439, t79474, t79864, t88675);
        let t89202 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1705(t1042, t15707, t19697, t23635, t23643, t23823, t23834, t42745, t42920, t42921, t4879, t53704, t53707, t6302, t66547, t79546, t79548, t79553, t79564, t79575, t79580, t88695);
        let (t89240, t89245, t89250) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1706(t6258, t6299, t1651, t23820, t1042, t1045, t1063, t11859, t15618, t16067, t16199, t16208, t19501, t23908, t23921, t24009, t3092, t3115, t3117, t3155, t43174, t4837, t4872, t4892, t53800, t5819, t5825, t6244, t6266, t66721, t66763, t78496, t79638, t88732, t88804);
        let t89283 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1707(t1011, t1012, t1015, t15707, t1675, t19968, t23485, t23859, t23911, t23976, t23980, t3091, t3092, t42518, t43223, t4834, t54687, t6323, t6327, t79559, t79742, t79744, t79758, t87126, t87145);
        let t89306 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1708(t1011, t1012, t11859, t3117, t3155, t42508, t6271, t6299, t67015, t67186, t67195, t67206, t79811, t79818, t79874, t79881, t79892, t79938, t79944, t79946, t87145);
        let (t89312, t89320, t89351) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1709(t6258, t6244, t1011, t1012, t1042, t15906, t19450, t23898, t3092, t3117, t3205, t3236, t3253, t371, t372, t373, t42731, t42977, t42978, t43155, t55247, t6339, t67473, t67502, t67575, t78496, t79957, t80038, t80113, t80277, t87107, t87145, t88695);
        let t89355 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1710(t88727, t88763, t88800, t88849, t88898, t88944, t89009, t89046, t89094, t89121, t89180, t89202, t89250, t89283, t89306, t89351);
        let t89397 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1711(t1076, t1079, t16312, t16313, t1647, t16600, t1695, t1696, t19351, t20178, t225, t23599, t23617, t23620, t23621, t24044, t24048, t24061, t24177, t3058, t3269, t342, t385, t4747, t4778, t4935, t6235, t6244, t6345, t6351, t6392, t6393, t80983, t81052, t89355);
        let t89437 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1712(t1076, t11121, t1652, t1680, t20175, t20178, t20191, t20204, t20211, t23603, t23959, t3058, t42060, t4752, t6245, t6251, t6259, t6350, t6351, t6392, t64687, t68022, t68144, t80173, t80810, t80901, t80921, t89312, t89320, t996);
        let (t89490, t89503, t89507) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1713(t6305, t6343, t378, t88714, t1678, t23640, t1082, t1087, t1089, t12047, t12052, t12149, t1668, t1689, t19446, t24042, t24108, t3204, t3299, t3304, t3317, t3318, t43154, t4954, t4975, t6258, t80243, t88815, t89312, t89320);
    (t89158, t89240, t89245, t89355, t89397, t89437, t89490, t89503, t89507)
}
