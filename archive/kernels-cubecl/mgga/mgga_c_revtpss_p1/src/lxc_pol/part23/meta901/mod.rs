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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta901<F: Float>(t1568: F, t6016: F, t231: F, t2782: F, t2783: F, t2723: F, t4503: F, t76169: F, t14568: F, t18726: F, t10871: F, t14545: F, t40271: F, t40294: F, t4514: F, t51507: F, t62777: F, t62809: F, t76127: F, t76136: F, t837: F, t18615: F, t18632: F, t18677: F, t40314: F, t40316: F, t4494: F, t4504: F, t51396: F, t51513: F, t6022: F, t62840: F, t62843: F, t62847: F, t62853: F, t820: F, t23359: F, t822: F, t18681: F, t40318: F, t4366: F, t51522: F, t51538: F, t51547: F, t62866: F, t62872: F, t62874: F, t62881: F, t14546: F, t1559: F, t40922: F, t51578: F, t51588: F, t51604: F, t51615: F, t62612: F, t62952: F, t62961: F, t62968: F, t76726: F, t77120: F, t879: F, t18714: F, t4424: F, t51635: F, t51637: F, t51646: F, t51657: F, t62983: F, t62987: F, t62992: F, t62999: F, t40945: F, t40958: F, t51660: F, t51676: F, t51683: F, t51685: F, t51686: F, t51688: F, t51704: F, t63015: F, t76131: F, t14507: F, t18313: F, t18525: F, t18699: F, t213: F, t23160: F, t23177: F, t234: F, t2770: F, t2815: F, t39581: F, t39633: F, t39635: F, t39723: F, t39724: F, t41095: F, t41098: F, t41102: F, t41105: F, t4474: F, t4533: F, t51264: F, t51269: F, t51471: F, t51484: F, t51553: F, t51561: F, t51565: F, t51727: F, t6071: F, t62577: F, t62583: F, t62587: F, t62591: F, t62595: F, t62601: F, t62641: F, t62665: F, t62667: F, t62670: F, t62675: F, t62763: F, t62775: F, t62788: F, t62907: F, t62909: F, t62920: F, t62922: F, t62938: F, t63050: F, t63053: F, t63058: F, t63062: F, t63064: F, t76081: F, t76100: F, t76104: F, t76108: F, t76147: F, t76174: F, t76198: F, t76206: F, t76247: F, t76264: F, t76275: F, t77151: F, t865: F, t868: F, t1580: F, t18663: F, t18785: F, t18800: F, t225: F, t23413: F, t257: F, t41078: F, t41118: F, t4534: F, t51733: F, t51742: F, t51756: F, t63085: F, t63091: F, t63094: F, t63099: F, t63103: F, t63109: F, t886: F, t887: F, t262: F, t5966: F, t23148: F, t23124: F, t39429: F, t39432: F, t39442: F, t4541: F, t49877: F, t50080: F, t76937: F, t76938: F, t76939: F, t76940: F, t76941: F, t775: F, t23421: F, t2411: F, t1940: F, t23429: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t41154: F, t76955: F, t76957: F, t76960: F, t890: F, t11064: F, t18268: F, t2403: F, t39756: F, t39760: F, t39764: F, t39770: F, t39773: F, t4343: F, t49930: F, t76967: F, t76969: F, t76970: F, t76973: F) -> (F, F, F, F, F, F, F) {
        let (t77159, t77171, t77177, t77183, t77191) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2867::<F>(t1568, t6016, t231, t2782, t2783, t2723, t4503, t76169, t14568, t18726, t10871, t14545);
        let t77193 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2868::<F>(t40271, t40294, t4514, t51507, t62777, t62809, t76127, t76136, t77171, t77177, t77183, t77191, t837);
        let t77213 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2869::<F>(t231, t2782, t2783, t76127, t18615, t18632, t18677, t2723, t40314, t40316, t4494, t4504, t51396, t51513, t6022, t62840, t62843, t62847, t62853, t820);
        let t77229 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2870::<F>(t23359, t822, t18632, t18681, t40318, t4366, t4504, t51522, t51538, t51547, t62866, t62872, t62874, t62881, t76169, t820, t837);
        let t77259 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2871::<F>(t14546, t1559, t18677, t40922, t4514, t51578, t51588, t51604, t51615, t62612, t62952, t62961, t62968, t76726, t77120, t820, t879);
        let t77278 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2872::<F>(t18677, t18714, t4424, t4514, t51635, t51637, t51646, t51657, t62983, t62987, t62992, t62999, t76169, t820, t837);
        let t77289 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2873::<F>(t40945, t40958, t4366, t4504, t51660, t51676, t51683, t51685, t51686, t51688, t51704, t63015, t76131);
        let t77298 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2874::<F>(t14507, t14546, t1559, t18313, t18525, t18699, t213, t23160, t23177, t234, t2770, t2815, t39581, t39633, t39635, t39723, t39724, t41095, t41098, t41102, t41105, t4366, t4424, t4474, t4504, t4514, t4533, t51264, t51269, t51471, t51484, t51553, t51561, t51565, t51727, t6071, t62577, t62583, t62587, t62591, t62595, t62601, t62641, t62665, t62667, t62670, t62675, t62763, t62775, t62788, t62907, t62909, t62920, t62922, t62938, t63050, t63053, t63058, t63062, t63064, t76081, t76100, t76104, t76108, t76147, t76169, t76174, t76198, t76206, t76247, t76264, t76275, t77151, t77159, t77193, t77213, t77229, t77259, t77278, t77289, t820, t837, t865, t868);
        let t77326 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2875::<F>(t213, t23359, t1580, t18663, t18785, t18800, t225, t23413, t257, t41078, t41118, t4474, t4534, t51733, t51742, t51756, t63085, t63091, t63094, t63099, t63103, t63109, t77151, t865, t886, t887);
        let (t77333, t77347) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2876::<F>(t262, t5966, t23148, t23124, t39429, t39432, t39442, t4541, t49877, t50080, t76937, t76938, t76939, t76940, t76941, t775);
        let t77360 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2877::<F>(t23421, t2411, t1940, t23429, t39520, t39528, t39531, t39534, t39537, t39540, t41154, t76955, t76957, t76960, t890);
        let (t77373, t77381) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2878::<F>(t11064, t23429, t18268, t2403, t39756, t39760, t39764, t39770, t39773, t4343, t49930, t76967, t76969, t76970, t76973);
    (t77298, t77326, t77333, t77347, t77360, t77373, t77381)
}
