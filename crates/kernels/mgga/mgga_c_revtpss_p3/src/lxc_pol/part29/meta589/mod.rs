//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1955;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta589<F: Float>(t1923: F, t26204: F, t7719: F, t101214: F, t2047: F, t101218: F, t101237: F, t101240: F, t101243: F, t101303: F, t101376: F, t2048: F, t25117: F, t25162: F, t26182: F, t28154: F, t28628: F, t28635: F, t6954: F, t7964: F, t92588: F, t95303: F, t95296: F, t28147: F, t95319: F, t28150: F, t7348: F, t101200: F, t101204: F, t101230: F, t101234: F, t101252: F, t101399: F, t26175: F, t92565: F, t95276: F, t95306: F, t95316: F, t95340: F, t5: F, t101805: F, t101824: F, t101849: F, t101875: F, t101896: F, t101919: F, t117: F, t7535: F, t9593: F, t101767: F, t1310: F, t13425: F, t13532: F, t13540: F, t13544: F, t1843: F, t2056: F, t2089: F, t2322: F, t26154: F, t26399: F, t26676: F, t27123: F, t28196: F, t28198: F, t28652: F, t28658: F, t28696: F, t4246: F, t4248: F, t4254: F, t4293: F, t508: F, t5517: F, t651: F, t7359: F, t7367: F, t7373: F, t7474: F, t98484: F, t98487: F, t530: F, t8107: F, t116: F, t28651: F, t13537: F, t13867: F, t2014: F, t22496: F, t2328: F, t25082: F, t25865: F, t26218: F, t26223: F, t26405: F, t26412: F, t27126: F, t28167: F, t28287: F, t28711: F, t28734: F, t33183: F, t35312: F, t3813: F, t4292: F, t49582: F, t5627: F, t671: F, t7374: F, t75353: F, t7732: F, t7898: F, t7983: F, t8065: F, t9069: F, t98588: F, t2106: F, t47672: F, t2028: F, t28911: F, t25894: F, t97680: F, t25875: F, t96236: F, t97688: F, t26304: F, t97705: F, t96187: F, t97685: F, t136: F, t2457: F, t8103: F, t25944: F, t25950: F, t28845: F, t14268: F, t2097: F, t7295: F, t7296: F, t96188: F, t96193: F, t96195: F, t96197: F, t28780: F, t94886: F, t28889: F, t686: F, t72: F, t7284: F, t10073: F, t1903: F, t2102: F, t25929: F, t28837: F, t3920: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t101949 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1951::<F>(t1923, t26204, t7719, t101214, t2047, t101218, t101237, t101240, t101243, t101303, t101376, t2048, t25117, t25162, t26182, t28154, t28628, t28635, t6954, t7964, t92588, t95303);
        let t101975 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1952::<F>(t28154, t95296, t28147, t95319, t28150, t7348, t25162, t101200, t101204, t101230, t101234, t101252, t101399, t26175, t26182, t28628, t92565, t95276, t95306, t95316, t95340);
        let (t101980, t102009) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1953::<F>(t5, t101805, t101824, t101849, t101875, t101896, t101919, t101949, t101975, t117, t7535, t9593, t101767, t1310, t13425, t13532, t13540, t13544, t1843, t2056, t2089, t2322, t26154, t26399, t26676, t27123, t28196, t28198, t28652, t28658, t28696, t4246, t4248, t4254, t4293, t508, t5517, t651, t7359, t7367, t7373, t7474, t98484, t98487);
        let (t102019, t102058) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1954::<F>(t530, t8107, t116, t28651, t13537, t13867, t2014, t22496, t2322, t2328, t25082, t25865, t26218, t26223, t26405, t26412, t27126, t28167, t28287, t28711, t28734, t33183, t35312, t3813, t4248, t4254, t4292, t49582, t5627, t651, t671, t7359, t7374, t7474, t75353, t7732, t7898, t7983, t8065, t9069, t98588);
        let (t102070, t102081, t102084, t102086, t102090, t102093, t102096) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1955::<F>(t2106, t47672, t2028, t28911, t25894, t97680, t25875, t96236, t97688, t26304, t97705, t96187, t97685);
        let (t102100, t102111) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956::<F>(t96236, t97685, t136, t2457, t8103, t25944, t25950, t28845, t102081, t102084, t102086, t102090, t102093, t102096, t14268, t2097, t7295, t7296, t96188, t96193, t96195, t96197);
        let (t102113, t102115, t102117, t102120, t102122) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1957::<F>(t28780, t94886, t28889, t686, t72, t7284, t10073, t1903, t2102, t25929, t28837, t3920);
    (t101980, t102009, t102019, t102058, t102070, t102100, t102111, t102113, t102115, t102117, t102120, t102122)
}
