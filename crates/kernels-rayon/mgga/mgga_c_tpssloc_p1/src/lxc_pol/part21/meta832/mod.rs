//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta832 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta832(t17579: f64, t225: f64, t18048: f64, t210: f64, t974: f64, t2985: f64, t1597: f64, t976: f64, t17826: f64, t2960: f64, t12652: f64, t4337: f64, t10236: f64, t17686: f64, t43070: f64, t10254: f64, t17635: f64, t13554: f64, t10263: f64, t13835: f64, t1539: f64, t17844: f64, t23547: f64, t2986: f64, t2988: f64, t43069: f64, t4510: f64, t4518: f64, t4531: f64, t47907: f64, t5845: f64, t984: f64, t17691: f64, t41831: f64, t41863: f64, t41870: f64, t41872: f64, t48087: f64, t48096: f64, t48098: f64, t48103: f64, t48116: f64, t60091: f64, t60153: f64, t60156: f64, t48155: f64, t48157: f64, t48159: f64, t48161: f64, t48163: f64, t48165: f64, t48167: f64, t60163: f64, t60166: f64, t60168: f64, t60171: f64, t60173: f64, t60189: f64, t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64, t60207: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64, t43002: f64, t60274: f64, t60277: f64, t60282: f64, t60296: f64, t60308: f64, t60310: f64, t60312: f64, t60315: f64, t60318: f64, t60321: f64, t60324: f64, t60327: f64, t135: f64, t17843: f64, t973: f64, t13831: f64, t17804: f64, t340: f64, t343: f64, t42811: f64, t42817: f64, t42873: f64, t42877: f64, t42893: f64, t42895: f64, t47887: f64, t47938: f64, t10189: f64, t5842: f64, t2990: f64, t13847: f64, t13861: f64, t17841: f64, t2987: f64, t10186: f64, t13812: f64, t17788: f64, t17805: f64, t17811: f64, t17814: f64, t17818: f64, t17854: f64, t17867: f64, t47966: f64, t48184: f64, t13798: f64, t13851: f64, t17791: f64, t17821: f64, t42903: f64, t42906: f64, t42911: f64, t42914: f64, t59668: f64, t59672: f64, t59696: f64, t59725: f64, t59742: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61058, t61061, t61064, t61065, t61066, t61074, t61078) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932(t17579, t225, t18048, t210, t974, t2985, t1597, t976, t17826, t2960, t12652, t4337);
        let (t61098, t61102) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933(t10236, t17686, t43070, t10254, t17635, t12652, t13554, t10263, t13835, t1539, t17844, t23547, t2960, t2986, t2988, t43069, t4510, t4518, t4531, t47907, t5845, t61065, t61066, t61074, t61078, t984);
        let (t61103, t61124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934(t10254, t17691, t41831, t41863, t41870, t41872, t48087, t48096, t48098, t48103, t48116, t60091, t60153, t60156);
        let t61138 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t60163, t60166, t60168, t60171, t60173, t60189);
        let t61150 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936(t60192, t60194, t60197, t60200, t60202, t60204, t60207, t60223, t60226, t60229, t60232, t60235);
        let t61163 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937(t43002, t60274, t60277, t60282, t60296, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let t61181 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938(t135, t17843, t973, t13831, t17804, t2986, t2988, t340, t343, t42811, t42817, t42873, t42877, t42893, t42895, t4531, t47887, t47938, t61103, t61124, t61138, t61150, t61163, t974);
        let t61214 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939(t10189, t5842, t2986, t2990, t13847, t13861, t17841, t2987, t10186, t13812, t17788, t17805, t17811, t17814, t17818, t17854, t17867, t4531, t47966, t48184);
        let t61241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940(t10186, t13798, t13851, t13861, t17791, t17821, t2986, t42903, t42906, t42911, t42914, t4510, t4518, t59668, t59672, t59696, t59725, t59742);
    (t61058, t61061, t61064, t61065, t61078, t61098, t61102, t61181, t61214, t61241)
}
