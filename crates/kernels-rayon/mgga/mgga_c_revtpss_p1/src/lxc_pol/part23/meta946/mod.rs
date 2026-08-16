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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta946(t81904: f64, t81917: f64, t81931: f64, t81944: f64, t81957: f64, t81969: f64, t81983: f64, t81995: f64, t1180: f64, t1187: f64, t1188: f64, t12553: f64, t17023: f64, t17032: f64, t20537: f64, t20615: f64, t20619: f64, t20678: f64, t24375: f64, t24376: f64, t24408: f64, t3491: f64, t45064: f64, t45188: f64, t45190: f64, t5158: f64, t5180: f64, t58242: f64, t6538: f64, t81591: f64, t81593: f64, t81596: f64, t81599: f64, t81601: f64, t81604: f64, t1168: f64, t12423: f64, t12429: f64, t12470: f64, t12511: f64, t1744: f64, t1745: f64, t20520: f64, t20542: f64, t20612: f64, t20618: f64, t20622: f64, t20626: f64, t24331: f64, t24366: f64, t24417: f64, t24420: f64, t3452: f64, t3477: f64, t45085: f64, t5142: f64, t5143: f64, t58005: f64, t58304: f64, t6487: f64, t6502: f64, t6506: f64, t69411: f64, t69565: f64, t57944: f64, t81612: f64, t81614: f64, t81618: f64, t81621: f64, t81623: f64, t81625: f64, t81627: f64, t81629: f64, t81631: f64, t81633: f64, t81635: f64, t81638: f64, t81641: f64, t81646: f64, t1179: f64, t24252: f64, t20641: f64, t57854: f64, t45232: f64, t68255: f64, t68257: f64, t68262: f64, t68277: f64, t81156: f64, t81158: f64, t81162: f64, t81167: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64, t43888: f64, t56236: f64, t58607: f64, t58609: f64, t58624: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64, t1189: f64, t16997: f64, t17026: f64, t20606: f64, t20609: f64, t20668: f64, t24431: f64, t435: f64, t5125: f64, t5147: f64, t58345: f64, t6503: f64, t6534: f64, t69376: f64, t69488: f64, t81649: f64, t81653: f64, t81656: f64, t81660: f64, t300: f64, t81781: f64, t81796: f64, t81835: f64, t81877: f64, t24864: f64, t460: f64, t5219: f64, t6695: f64, t1210: f64, t1214: f64, t1215: f64, t12628: f64, t1274: f64, t1277: f64, t1294: f64, t1295: f64, t13182: f64, t1770: f64, t18065: f64, t18097: f64, t1828: f64, t1829: f64, t20697: f64, t20744: f64, t20748: f64, t20759: f64, t21082: f64, t21344: f64, t21366: f64, t21394: f64, t24515: f64, t24616: f64, t25015: f64, t3556: f64, t5216: f64, t5220: f64, t5246: f64, t5423: f64, t5497: f64, t56314: f64, t56315: f64, t56332: f64, t56416: f64, t6574: f64, t6588: f64, t6697: f64, t6702: f64, t6745: f64, t72927: f64, t73187: f64, t1811: f64, t20849: f64, t1774: f64, t6564: f64, t1204: f64, t1211: f64, t17973: f64, t17974: f64, t17995: f64, t18005: f64, t18059: f64, t20703: f64, t20704: f64, t20709: f64, t20714: f64, t20756: f64, t21617: f64, t21624: f64, t24519: f64, t24525: f64, t24866: f64, t24900: f64, t34934: f64, t3561: f64, t3567: f64, t3572: f64, t3737: f64, t5498: f64, t56327: f64, t56588: f64, t6573: f64, t6744: f64, t72805: f64, t12641: f64, t17986: f64, t1813: f64, t20700: f64, t20728: f64, t20753: f64, t21333: f64, t21347: f64, t21382: f64, t21408: f64, t24633: f64, t24892: f64, t3736: f64, t5225: f64, t5231: f64, t5251: f64, t5428: f64, t5429: f64, t56393: f64, t6580: f64, t6587: f64, t6703: f64, t72767: f64, t72784: f64, t72843: f64, t12772: f64, t24568: f64, t5340: f64, t24572: f64, t5331: f64, t11249: f64, t24543: f64, t12910: f64, t17459: f64, t17753: f64, t1808: f64, t20952: f64, t21004: f64, t21014: f64, t21017: f64, t21030: f64, t21173: f64, t21242: f64, t24751: f64, t3626: f64, t3629: f64, t3720: f64, t5397: f64, t57147: f64, t57382: f64, t69661: f64, t69710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81998, t82006) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3116(t81904, t81917, t81931, t81944, t81957, t81969, t81983, t81995, t1180, t1187, t1188, t12553, t17023, t17032, t20537, t20615, t20619, t20678, t24375, t24376, t24408, t3491, t45064, t45188, t45190, t5158, t5180, t58242, t6538, t81591, t81593, t81596, t81599, t81601, t81604);
        let t82045 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3117(t1168, t12423, t12429, t12470, t12511, t17032, t1744, t1745, t20520, t20542, t20612, t20618, t20622, t20626, t24331, t24366, t24417, t24420, t3452, t3477, t45085, t5142, t5143, t58005, t58304, t6487, t6502, t6506, t69411, t69565);
        let t82049 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3118(t20619, t57944, t81612, t81614, t81618, t81621, t81623, t81625, t81627, t81629, t81631, t81633, t81635, t81638, t81641, t81646);
        let (t82050, t82060, t82093) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3119(t1179, t24252, t20641, t57854, t45232, t68255, t68257, t68262, t68277, t81156, t81158, t81162, t81167, t81171, t81175, t81179, t81184, t81188, t81192, t81196, t81200, t81204, t81209, t81214);
        let t82111 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3120(t43888, t56236, t58607, t58609, t58624, t68332, t68334, t68336, t68389, t68399, t68454, t68456, t81224, t81228, t81230, t81232, t81234, t81236, t81242, t81245);
        let t82115 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3121(t1168, t1187, t1189, t12429, t12553, t16997, t17023, t17026, t17032, t20606, t20609, t20668, t24431, t435, t5125, t5147, t58345, t6503, t6534, t69376, t69488, t81649, t81653, t81656, t81660, t82050, t82060, t82093, t82111);
        let (t82119, t82147, t82150) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3122(t300, t81781, t81796, t81835, t81877, t82006, t82045, t82049, t82115, t24864, t460, t5219, t6695);
        let t82169 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3123(t1210, t1214, t1215, t12628, t1274, t1277, t1294, t1295, t13182, t1770, t18065, t18097, t1828, t1829, t20697, t20744, t20748, t20759, t21082, t21344, t21366, t21394, t24515, t24616, t25015, t3556, t5216, t5220, t5246, t5423, t5497, t56314, t56315, t56332, t56416, t6574, t6588, t6697, t6702, t6745, t72927, t73187, t82147, t82150);
        let (t82207, t82220) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3124(t1811, t20849, t1774, t21082, t6564, t1204, t1210, t1211, t1215, t1274, t1277, t1295, t17973, t17974, t17995, t18005, t18059, t20703, t20704, t20709, t20714, t20744, t20756, t21617, t21624, t24519, t24525, t24866, t24900, t34934, t3561, t3567, t3572, t3737, t5220, t5497, t5498, t56327, t56588, t6573, t6574, t6744, t6745, t72805);
        let t82266 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3125(t1770, t6695, t1210, t12641, t1277, t1294, t1295, t17986, t18005, t18065, t18097, t1813, t1829, t20697, t20700, t20728, t20748, t20753, t21333, t21347, t21382, t21408, t24633, t24892, t3736, t5220, t5225, t5231, t5251, t5423, t5428, t5429, t56393, t6580, t6587, t6703, t72767, t72784, t72843);
        let (t82286, t82289, t82293) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3126(t12772, t24568, t5340, t24572, t5331, t11249, t24543);
        let t82305 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3127(t12910, t17459, t17753, t1808, t20952, t21004, t21014, t21017, t21030, t21173, t21242, t24751, t3626, t3629, t3720, t5397, t57147, t57382, t69661, t69710, t82286, t82289, t82293);
    (t81998, t82060, t82119, t82169, t82207, t82220, t82266, t82293, t82305)
}
