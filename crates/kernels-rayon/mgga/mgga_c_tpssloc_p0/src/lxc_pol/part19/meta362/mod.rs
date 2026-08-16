//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta362 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1314;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1316;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1317;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1318;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1320;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1321;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta362(t10413: f64, t10414: f64, t10422: f64, t10393: f64, t3070: f64, t11046: f64, t42387: f64, t10457: f64, t820: f64, t10409: f64, t10936: f64, t3180: f64, t10390: f64, t10394: f64, t10398: f64, t1041: f64, t10428: f64, t10433: f64, t10884: f64, t10891: f64, t10904: f64, t10915: f64, t10919: f64, t10932: f64, t14187: f64, t2960: f64, t3048: f64, t3071: f64, t3073: f64, t42460: f64, t42468: f64, t4582: f64, t884: f64, t10401: f64, t10935: f64, t3186: f64, t3200: f64, t11051: f64, t3069: f64, t10454: f64, t10459: f64, t3036: f64, t3087: f64, t3033: f64, t3128: f64, t10987: f64, t135: f64, t973: f64, t10405: f64, t10408: f64, t10415: f64, t10937: f64, t10944: f64, t10957: f64, t10988: f64, t2771: f64, t2780: f64, t3064: f64, t3121: f64, t3134: f64, t10402: f64, t11034: f64, t11037: f64, t2402: f64, t999: f64, t9277: f64, t972: f64, t10263: f64, t3139: f64, t1030: f64, t10477: f64, t10472: f64, t10475: f64, t10903: f64, t10948: f64, t10890: f64, t10898: f64, t3103: f64, t1000: f64, t10410: f64, t10485: f64, t10860: f64, t10879: f64, t3043: f64, t3109: f64, t3117: f64, t3123: f64, t11002: f64, t10508: f64, t248: f64, t3130: f64, t3132: f64, t10969: f64, t121: f64, t10305: f64, t1015: f64, t3142: f64, t698: f64, t3147: f64, t10981: f64, t1044: f64, t10972: f64, t3057: f64, t3098: f64, t3114: f64, t3143: f64, t3148: f64, t41709: f64, t10984: f64, t10213: f64, t41687: f64, t10857: f64, t376: f64, t1004: f64, t10956: f64, t10863: f64, t3053: f64, t10516: f64, t3113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42478, t42481, t42483, t42490, t42496) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1314(t10413, t10414, t10422, t10393, t3070, t11046, t42387, t10457, t820, t10409, t10936, t3180);
        let t42499 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315(t10390, t10394, t10398, t1041, t10428, t10433, t10884, t10891, t10904, t10915, t10919, t10932, t14187, t2960, t3048, t3071, t3073, t42460, t42468, t42478, t42481, t42483, t42490, t42496, t4582, t884);
        let (t42505, t42508, t42511, t42514, t42518, t42520, t42522) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1316(t10401, t10935, t3186, t3200, t11051, t3069, t10454, t3048, t10459, t3036, t3087, t3033, t3128);
        let t42540 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1317(t10987, t135, t973, t10394, t10405, t10408, t10415, t10937, t10944, t10957, t10988, t2771, t2780, t2960, t3064, t3070, t3071, t3073, t3121, t3134, t42505, t42508, t42511, t42514, t42518, t42522);
        let (t42541, t42546, t42552, t42554, t42557, t42559) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1318(t10402, t11034, t11037, t2402, t973, t999, t9277, t972, t10263, t3139, t1030, t10477);
        let t42580 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319(t10472, t10475, t42559, t3128, t10903, t10948, t10890, t10898, t3103, t1000, t10390, t10405, t10410, t10415, t10485, t10860, t10879, t10919, t3043, t3109, t3117, t3123, t3134, t42541, t42546, t42552, t42554, t42557);
        let (t42582, t42586, t42595, t42600) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1320(t10904, t11002, t10508, t248, t3130, t3132, t10969, t121, t10305, t1041, t1015, t3033, t42520);
        let t42621 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1321(t3142, t698, t973, t3147, t10981, t2960, t10263, t1041, t1044, t10860, t10957, t10972, t248, t3043, t3048, t3057, t3098, t3114, t3143, t3148, t41709, t42582, t42586, t42595, t42600);
        let (t42622, t42624, t42639, t42648, t42651, t42653) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1322(t10984, t2960, t10213, t41687, t10857, t376, t1004, t10956, t10863, t3053, t10516, t3113);
    (t42499, t42540, t42554, t42559, t42580, t42621, t42622, t42624, t42639, t42648, t42651, t42653)
}
