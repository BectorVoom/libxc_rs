//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta901 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2867;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2868;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2869;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2870;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2871;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2872;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2873;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2874;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2875;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2876;
use chunk10::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2877;
use chunk11::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2878;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta901(t1568: f64, t6016: f64, t231: f64, t2782: f64, t2783: f64, t2723: f64, t4503: f64, t76169: f64, t14568: f64, t18726: f64, t10871: f64, t14545: f64, t40271: f64, t40294: f64, t4514: f64, t51507: f64, t62777: f64, t62809: f64, t76127: f64, t76136: f64, t837: f64, t18615: f64, t18632: f64, t18677: f64, t40314: f64, t40316: f64, t4494: f64, t4504: f64, t51396: f64, t51513: f64, t6022: f64, t62840: f64, t62843: f64, t62847: f64, t62853: f64, t820: f64, t23359: f64, t822: f64, t18681: f64, t40318: f64, t4366: f64, t51522: f64, t51538: f64, t51547: f64, t62866: f64, t62872: f64, t62874: f64, t62881: f64, t14546: f64, t1559: f64, t40922: f64, t51578: f64, t51588: f64, t51604: f64, t51615: f64, t62612: f64, t62952: f64, t62961: f64, t62968: f64, t76726: f64, t77120: f64, t879: f64, t18714: f64, t4424: f64, t51635: f64, t51637: f64, t51646: f64, t51657: f64, t62983: f64, t62987: f64, t62992: f64, t62999: f64, t40945: f64, t40958: f64, t51660: f64, t51676: f64, t51683: f64, t51685: f64, t51686: f64, t51688: f64, t51704: f64, t63015: f64, t76131: f64, t14507: f64, t18313: f64, t18525: f64, t18699: f64, t213: f64, t23160: f64, t23177: f64, t234: f64, t2770: f64, t2815: f64, t39581: f64, t39633: f64, t39635: f64, t39723: f64, t39724: f64, t41095: f64, t41098: f64, t41102: f64, t41105: f64, t4474: f64, t4533: f64, t51264: f64, t51269: f64, t51471: f64, t51484: f64, t51553: f64, t51561: f64, t51565: f64, t51727: f64, t6071: f64, t62577: f64, t62583: f64, t62587: f64, t62591: f64, t62595: f64, t62601: f64, t62641: f64, t62665: f64, t62667: f64, t62670: f64, t62675: f64, t62763: f64, t62775: f64, t62788: f64, t62907: f64, t62909: f64, t62920: f64, t62922: f64, t62938: f64, t63050: f64, t63053: f64, t63058: f64, t63062: f64, t63064: f64, t76081: f64, t76100: f64, t76104: f64, t76108: f64, t76147: f64, t76174: f64, t76198: f64, t76206: f64, t76247: f64, t76264: f64, t76275: f64, t77151: f64, t865: f64, t868: f64, t1580: f64, t18663: f64, t18785: f64, t18800: f64, t225: f64, t23413: f64, t257: f64, t41078: f64, t41118: f64, t4534: f64, t51733: f64, t51742: f64, t51756: f64, t63085: f64, t63091: f64, t63094: f64, t63099: f64, t63103: f64, t63109: f64, t886: f64, t887: f64, t262: f64, t5966: f64, t23148: f64, t23124: f64, t39429: f64, t39432: f64, t39442: f64, t4541: f64, t49877: f64, t50080: f64, t76937: f64, t76938: f64, t76939: f64, t76940: f64, t76941: f64, t775: f64, t23421: f64, t2411: f64, t1940: f64, t23429: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t41154: f64, t76955: f64, t76957: f64, t76960: f64, t890: f64, t11064: f64, t18268: f64, t2403: f64, t39756: f64, t39760: f64, t39764: f64, t39770: f64, t39773: f64, t4343: f64, t49930: f64, t76967: f64, t76969: f64, t76970: f64, t76973: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t77159, t77171, t77177, t77183, t77191) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2867(t1568, t6016, t231, t2782, t2783, t2723, t4503, t76169, t14568, t18726, t10871, t14545);
        let t77193 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2868(t40271, t40294, t4514, t51507, t62777, t62809, t76127, t76136, t77171, t77177, t77183, t77191, t837);
        let t77213 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2869(t231, t2782, t2783, t76127, t18615, t18632, t18677, t2723, t40314, t40316, t4494, t4504, t51396, t51513, t6022, t62840, t62843, t62847, t62853, t820);
        let t77229 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2870(t23359, t822, t18632, t18681, t40318, t4366, t4504, t51522, t51538, t51547, t62866, t62872, t62874, t62881, t76169, t820, t837);
        let t77259 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2871(t14546, t1559, t18677, t40922, t4514, t51578, t51588, t51604, t51615, t62612, t62952, t62961, t62968, t76726, t77120, t820, t879);
        let t77278 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2872(t18677, t18714, t4424, t4514, t51635, t51637, t51646, t51657, t62983, t62987, t62992, t62999, t76169, t820, t837);
        let t77289 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2873(t40945, t40958, t4366, t4504, t51660, t51676, t51683, t51685, t51686, t51688, t51704, t63015, t76131);
        let t77298 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2874(t14507, t14546, t1559, t18313, t18525, t18699, t213, t23160, t23177, t234, t2770, t2815, t39581, t39633, t39635, t39723, t39724, t41095, t41098, t41102, t41105, t4366, t4424, t4474, t4504, t4514, t4533, t51264, t51269, t51471, t51484, t51553, t51561, t51565, t51727, t6071, t62577, t62583, t62587, t62591, t62595, t62601, t62641, t62665, t62667, t62670, t62675, t62763, t62775, t62788, t62907, t62909, t62920, t62922, t62938, t63050, t63053, t63058, t63062, t63064, t76081, t76100, t76104, t76108, t76147, t76169, t76174, t76198, t76206, t76247, t76264, t76275, t77151, t77159, t77193, t77213, t77229, t77259, t77278, t77289, t820, t837, t865, t868);
        let t77326 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2875(t213, t23359, t1580, t18663, t18785, t18800, t225, t23413, t257, t41078, t41118, t4474, t4534, t51733, t51742, t51756, t63085, t63091, t63094, t63099, t63103, t63109, t77151, t865, t886, t887);
        let (t77333, t77347) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2876(t262, t5966, t23148, t23124, t39429, t39432, t39442, t4541, t49877, t50080, t76937, t76938, t76939, t76940, t76941, t775);
        let t77360 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2877(t23421, t2411, t1940, t23429, t39520, t39528, t39531, t39534, t39537, t39540, t41154, t76955, t76957, t76960, t890);
        let (t77373, t77381) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2878(t11064, t23429, t18268, t2403, t39756, t39760, t39764, t39770, t39773, t4343, t49930, t76967, t76969, t76970, t76973);
    (t77298, t77326, t77333, t77347, t77360, t77373, t77381)
}
