//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1955;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta589(t1923: f64, t26204: f64, t7719: f64, t101214: f64, t2047: f64, t101218: f64, t101237: f64, t101240: f64, t101243: f64, t101303: f64, t101376: f64, t2048: f64, t25117: f64, t25162: f64, t26182: f64, t28154: f64, t28628: f64, t28635: f64, t6954: f64, t7964: f64, t92588: f64, t95303: f64, t95296: f64, t28147: f64, t95319: f64, t28150: f64, t7348: f64, t101200: f64, t101204: f64, t101230: f64, t101234: f64, t101252: f64, t101399: f64, t26175: f64, t92565: f64, t95276: f64, t95306: f64, t95316: f64, t95340: f64, t5: f64, t101805: f64, t101824: f64, t101849: f64, t101875: f64, t101896: f64, t101919: f64, t117: f64, t7535: f64, t9593: f64, t101767: f64, t1310: f64, t13425: f64, t13532: f64, t13540: f64, t13544: f64, t1843: f64, t2056: f64, t2089: f64, t2322: f64, t26154: f64, t26399: f64, t26676: f64, t27123: f64, t28196: f64, t28198: f64, t28652: f64, t28658: f64, t28696: f64, t4246: f64, t4248: f64, t4254: f64, t4293: f64, t508: f64, t5517: f64, t651: f64, t7359: f64, t7367: f64, t7373: f64, t7474: f64, t98484: f64, t98487: f64, t530: f64, t8107: f64, t116: f64, t28651: f64, t13537: f64, t13867: f64, t2014: f64, t22496: f64, t2328: f64, t25082: f64, t25865: f64, t26218: f64, t26223: f64, t26405: f64, t26412: f64, t27126: f64, t28167: f64, t28287: f64, t28711: f64, t28734: f64, t33183: f64, t35312: f64, t3813: f64, t4292: f64, t49582: f64, t5627: f64, t671: f64, t7374: f64, t75353: f64, t7732: f64, t7898: f64, t7983: f64, t8065: f64, t9069: f64, t98588: f64, t2106: f64, t47672: f64, t2028: f64, t28911: f64, t25894: f64, t97680: f64, t25875: f64, t96236: f64, t97688: f64, t26304: f64, t97705: f64, t96187: f64, t97685: f64, t136: f64, t2457: f64, t8103: f64, t25944: f64, t25950: f64, t28845: f64, t14268: f64, t2097: f64, t7295: f64, t7296: f64, t96188: f64, t96193: f64, t96195: f64, t96197: f64, t28780: f64, t94886: f64, t28889: f64, t686: f64, t72: f64, t7284: f64, t10073: f64, t1903: f64, t2102: f64, t25929: f64, t28837: f64, t3920: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t101949 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951(t1923, t26204, t7719, t101214, t2047, t101218, t101237, t101240, t101243, t101303, t101376, t2048, t25117, t25162, t26182, t28154, t28628, t28635, t6954, t7964, t92588, t95303);
        let t101975 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952(t28154, t95296, t28147, t95319, t28150, t7348, t25162, t101200, t101204, t101230, t101234, t101252, t101399, t26175, t26182, t28628, t92565, t95276, t95306, t95316, t95340);
        let (t101980, t102009) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953(t5, t101805, t101824, t101849, t101875, t101896, t101919, t101949, t101975, t117, t7535, t9593, t101767, t1310, t13425, t13532, t13540, t13544, t1843, t2056, t2089, t2322, t26154, t26399, t26676, t27123, t28196, t28198, t28652, t28658, t28696, t4246, t4248, t4254, t4293, t508, t5517, t651, t7359, t7367, t7373, t7474, t98484, t98487);
        let (t102019, t102058) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954(t530, t8107, t116, t28651, t13537, t13867, t2014, t22496, t2322, t2328, t25082, t25865, t26218, t26223, t26405, t26412, t27126, t28167, t28287, t28711, t28734, t33183, t35312, t3813, t4248, t4254, t4292, t49582, t5627, t651, t671, t7359, t7374, t7474, t75353, t7732, t7898, t7983, t8065, t9069, t98588);
        let (t102070, t102081, t102084, t102086, t102090, t102093, t102096) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1955(t2106, t47672, t2028, t28911, t25894, t97680, t25875, t96236, t97688, t26304, t97705, t96187, t97685);
        let (t102100, t102111) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956(t96236, t97685, t136, t2457, t8103, t25944, t25950, t28845, t102081, t102084, t102086, t102090, t102093, t102096, t14268, t2097, t7295, t7296, t96188, t96193, t96195, t96197);
        let (t102113, t102115, t102117, t102120, t102122) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1957(t28780, t94886, t28889, t686, t72, t7284, t10073, t1903, t2102, t25929, t28837, t3920);
    (t101980, t102009, t102019, t102058, t102070, t102100, t102111, t102113, t102115, t102117, t102120, t102122)
}
