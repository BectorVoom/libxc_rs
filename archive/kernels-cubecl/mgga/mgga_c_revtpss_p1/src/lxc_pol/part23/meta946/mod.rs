//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta946 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3116;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3117;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3118;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3119;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3120;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3121;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3122;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3123;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3124;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3125;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3126;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3127;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta946<F: Float>(t81904: F, t81917: F, t81931: F, t81944: F, t81957: F, t81969: F, t81983: F, t81995: F, t1180: F, t1187: F, t1188: F, t12553: F, t17023: F, t17032: F, t20537: F, t20615: F, t20619: F, t20678: F, t24375: F, t24376: F, t24408: F, t3491: F, t45064: F, t45188: F, t45190: F, t5158: F, t5180: F, t58242: F, t6538: F, t81591: F, t81593: F, t81596: F, t81599: F, t81601: F, t81604: F, t1168: F, t12423: F, t12429: F, t12470: F, t12511: F, t1744: F, t1745: F, t20520: F, t20542: F, t20612: F, t20618: F, t20622: F, t20626: F, t24331: F, t24366: F, t24417: F, t24420: F, t3452: F, t3477: F, t45085: F, t5142: F, t5143: F, t58005: F, t58304: F, t6487: F, t6502: F, t6506: F, t69411: F, t69565: F, t57944: F, t81612: F, t81614: F, t81618: F, t81621: F, t81623: F, t81625: F, t81627: F, t81629: F, t81631: F, t81633: F, t81635: F, t81638: F, t81641: F, t81646: F, t1179: F, t24252: F, t20641: F, t57854: F, t45232: F, t68255: F, t68257: F, t68262: F, t68277: F, t81156: F, t81158: F, t81162: F, t81167: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F, t43888: F, t56236: F, t58607: F, t58609: F, t58624: F, t68332: F, t68334: F, t68336: F, t68389: F, t68399: F, t68454: F, t68456: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t81242: F, t81245: F, t1189: F, t16997: F, t17026: F, t20606: F, t20609: F, t20668: F, t24431: F, t435: F, t5125: F, t5147: F, t58345: F, t6503: F, t6534: F, t69376: F, t69488: F, t81649: F, t81653: F, t81656: F, t81660: F, t300: F, t81781: F, t81796: F, t81835: F, t81877: F, t24864: F, t460: F, t5219: F, t6695: F, t1210: F, t1214: F, t1215: F, t12628: F, t1274: F, t1277: F, t1294: F, t1295: F, t13182: F, t1770: F, t18065: F, t18097: F, t1828: F, t1829: F, t20697: F, t20744: F, t20748: F, t20759: F, t21082: F, t21344: F, t21366: F, t21394: F, t24515: F, t24616: F, t25015: F, t3556: F, t5216: F, t5220: F, t5246: F, t5423: F, t5497: F, t56314: F, t56315: F, t56332: F, t56416: F, t6574: F, t6588: F, t6697: F, t6702: F, t6745: F, t72927: F, t73187: F, t1811: F, t20849: F, t1774: F, t6564: F, t1204: F, t1211: F, t17973: F, t17974: F, t17995: F, t18005: F, t18059: F, t20703: F, t20704: F, t20709: F, t20714: F, t20756: F, t21617: F, t21624: F, t24519: F, t24525: F, t24866: F, t24900: F, t34934: F, t3561: F, t3567: F, t3572: F, t3737: F, t5498: F, t56327: F, t56588: F, t6573: F, t6744: F, t72805: F, t12641: F, t17986: F, t1813: F, t20700: F, t20728: F, t20753: F, t21333: F, t21347: F, t21382: F, t21408: F, t24633: F, t24892: F, t3736: F, t5225: F, t5231: F, t5251: F, t5428: F, t5429: F, t56393: F, t6580: F, t6587: F, t6703: F, t72767: F, t72784: F, t72843: F, t12772: F, t24568: F, t5340: F, t24572: F, t5331: F, t11249: F, t24543: F, t12910: F, t17459: F, t17753: F, t1808: F, t20952: F, t21004: F, t21014: F, t21017: F, t21030: F, t21173: F, t21242: F, t24751: F, t3626: F, t3629: F, t3720: F, t5397: F, t57147: F, t57382: F, t69661: F, t69710: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t81998, t82006) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3116::<F>(t81904, t81917, t81931, t81944, t81957, t81969, t81983, t81995, t1180, t1187, t1188, t12553, t17023, t17032, t20537, t20615, t20619, t20678, t24375, t24376, t24408, t3491, t45064, t45188, t45190, t5158, t5180, t58242, t6538, t81591, t81593, t81596, t81599, t81601, t81604);
        let t82045 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3117::<F>(t1168, t12423, t12429, t12470, t12511, t17032, t1744, t1745, t20520, t20542, t20612, t20618, t20622, t20626, t24331, t24366, t24417, t24420, t3452, t3477, t45085, t5142, t5143, t58005, t58304, t6487, t6502, t6506, t69411, t69565);
        let t82049 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3118::<F>(t20619, t57944, t81612, t81614, t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641, t81646);
        let (t82050, t82060, t82093) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3119::<F>(t1179, t24252, t20641, t57854, t45232, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t82111 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3120::<F>(t43888, t56236, t58607, t58609, t58624, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let t82115 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3121::<F>(t1168, t1187, t1189, t12429, t12553, t16997, t17023, t17026, t17032, t20606, t20609, t20668, t24431, t435, t5125, t5147, t58345, t6503, t6534, t69376, t69488, t81649, t81653, t81656, t81660, t82050, t82060, t82093, t82111);
        let (t82119, t82147, t82150) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3122::<F>(t300, t81781, t81796, t81835, t81877, t82006, t82045, t82049, t82115, t24864, t460, t5219, t6695);
        let t82169 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3123::<F>(t1210, t1214, t1215, t12628, t1274, t1277, t1294, t1295, t13182, t1770, t18065, t18097, t1828, t1829, t20697, t20744, t20748, t20759, t21082, t21344, t21366, t21394, t24515, t24616, t25015, t3556, t5216, t5220, t5246, t5423, t5497, t56314, t56315, t56332, t56416, t6574, t6588, t6697, t6702, t6745, t72927, t73187, t82147, t82150);
        let (t82207, t82220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3124::<F>(t1811, t20849, t1774, t21082, t6564, t1204, t1210, t1211, t1215, t1274, t1277, t1295, t17973, t17974, t17995, t18005, t18059, t20703, t20704, t20709, t20714, t20744, t20756, t21617, t21624, t24519, t24525, t24866, t24900, t34934, t3561, t3567, t3572, t3737, t5220, t5497, t5498, t56327, t56588, t6573, t6574, t6744, t6745, t72805);
        let t82266 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3125::<F>(t1770, t6695, t1210, t12641, t1277, t1294, t1295, t17986, t18005, t18065, t18097, t1813, t1829, t20697, t20700, t20728, t20748, t20753, t21333, t21347, t21382, t21408, t24633, t24892, t3736, t5220, t5225, t5231, t5251, t5423, t5428, t5429, t56393, t6580, t6587, t6703, t72767, t72784, t72843);
        let (t82286, t82289, t82293) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3126::<F>(t12772, t24568, t5340, t24572, t5331, t11249, t24543);
        let t82305 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3127::<F>(t12910, t17459, t17753, t1808, t20952, t21004, t21014, t21017, t21030, t21173, t21242, t24751, t3626, t3629, t3720, t5397, t57147, t57382, t69661, t69710, t82286, t82289, t82293);
    (t81998, t82060, t82119, t82169, t82207, t82220, t82266, t82293, t82305)
}
