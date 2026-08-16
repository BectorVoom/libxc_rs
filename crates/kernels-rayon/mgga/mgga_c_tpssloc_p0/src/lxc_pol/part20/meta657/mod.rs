//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta657 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2428;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2431;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2434;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2436;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2438;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta657(t13998: f64, t2960: f64, t42875: f64, t4338: f64, t973: f64, t10422: f64, t14040: f64, t3070: f64, t10516: f64, t4640: f64, t10403: f64, t14121: f64, t13748: f64, t1025: f64, t10884: f64, t10937: f64, t14041: f64, t1539: f64, t2780: f64, t3071: f64, t42483: f64, t42552: f64, t42557: f64, t42578: f64, t42582: f64, t4650: f64, t13965: f64, t3114: f64, t14202: f64, t3117: f64, t10423: f64, t13995: f64, t10413: f64, t14221: f64, t10949: f64, t14025: f64, t10195: f64, t10408: f64, t10433: f64, t10965: f64, t13991: f64, t14215: f64, t14511: f64, t1616: f64, t42541: f64, t42565: f64, t42570: f64, t42586: f64, t42861: f64, t4596: f64, t4636: f64, t47679: f64, t10883: f64, t13969: f64, t14106: f64, t13559: f64, t2970: f64, t1036: f64, t13942: f64, t3047: f64, t4616: f64, t10890: f64, t14507: f64, t1041: f64, t14188: f64, t1046: f64, t10898: f64, t13977: f64, t13982: f64, t13987: f64, t1618: f64, t3043: f64, t42595: f64, t43120: f64, t43322: f64, t43343: f64, t4652: f64, t1020: f64, t14489: f64, t248: f64, t3101: f64, t3038: f64, t49650: f64, t1022: f64, t10480: f64, t10876: f64, t13975: f64, t13985: f64, t14143: f64, t14180: f64, t14211: f64, t14218: f64, t2244: f64, t2775: f64, t2776: f64, t3132: f64, t360: f64, t42610: f64, t42613: f64, t42619: f64, t42622: f64, t42651: f64, t4582: f64, t13611: f64, t3051: f64, t14137: f64, t14125: f64, t4571: f64, t10508: f64, t10962: f64, t4630: f64, t13961: f64, t10863: f64, t14126: f64, t14213: f64, t14491: f64, t17732: f64, t3109: f64, t42508: f64, t43358: f64, t4575: f64, t884: f64, t10957: f64, t13950: f64, t3048: f64, t14173: f64, t247: f64, t677: f64, t4589: f64, t10969: f64, t41687: f64, t42600: f64, t42721: f64, t42729: f64, t42731: f64, t4583: f64, t4588: f64, t45993: f64, t4600: f64, t46006: f64, t48497: f64, t1009: f64, t13939: f64, t1011: f64, t1019: f64, t10868: f64, t4347: f64, t14134: f64, t14102: f64, t3039: f64, t13990: f64, t14093: f64, t42735: f64, t42752: f64, t43094: f64, t43097: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49658, t49662, t49666, t49678, t49682) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2428(t13998, t2960, t42875, t4338, t973, t10422, t14040, t3070, t10516, t4640, t10403, t14121);
        let t49688 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429(t13748, t2960, t1025, t10884, t10937, t14041, t1539, t2780, t3070, t3071, t42483, t42552, t42557, t42578, t42582, t4650, t49658, t49662, t49666, t49678, t49682);
        let t49718 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430(t13965, t3114, t14202, t3117, t10423, t13995, t10413, t10422, t14221, t10949, t14025, t10195, t10408, t10433, t10965, t13991, t14215, t14511, t1616, t3070, t42541, t42565, t42570, t42586, t42861, t4596, t4636, t47679, t973);
        let (t49721, t49732, t49734, t49740, t49743) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2431(t10883, t13969, t14106, t13559, t2970, t973, t1036, t13942, t3047, t4616, t10890, t14507);
        let t49750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432(t1041, t13969, t14188, t1046, t10898, t10949, t13977, t13982, t13987, t1618, t3043, t42595, t43120, t43322, t43343, t4596, t4652, t49721, t49732, t49734, t49740, t49743);
        let t49786 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433(t1020, t14489, t248, t3101, t3038, t49650, t1022, t10403, t10413, t10480, t10876, t13975, t13985, t14143, t14180, t14211, t14218, t2244, t2775, t2776, t3043, t3071, t3117, t3132, t360, t42610, t42613, t42619, t42622, t42651, t4582);
        let (t49799, t49801, t49808, t49810, t49818) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2434(t1041, t13611, t248, t3051, t14137, t3117, t10413, t10422, t14125, t10965, t4571, t1020, t10508, t4650);
        let t49824 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435(t49818, t10962, t4630, t13961, t3114, t10403, t10863, t14126, t14213, t14489, t14491, t17732, t3070, t3071, t3109, t42508, t43358, t4575, t4636, t49799, t49801, t49808, t49810, t884);
        let (t49827, t49829, t49832, t49846, t49850) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2436(t10957, t4571, t13950, t3048, t13965, t3109, t1041, t13969, t14173, t247, t677);
        let t49860 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437(t1041, t4589, t49850, t10969, t41687, t42600, t42721, t42729, t42731, t4582, t4583, t4588, t45993, t4600, t46006, t48497, t49827, t49829, t49832, t49846);
        let (t49864, t49866, t49872, t49873, t49877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2438(t1009, t13939, t1011, t1019, t1041, t10868, t248, t4347, t14134, t3117, t10863, t4571);
        let t49891 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439(t13969, t14102, t3039, t10876, t13990, t14134, t3048, t1025, t10957, t14093, t42735, t42752, t43094, t43097, t4636, t49866, t49872, t49873, t49877);
    (t49688, t49718, t49750, t49786, t49824, t49850, t49860, t49864, t49891)
}
