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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta362<F: Float>(t10413: F, t10414: F, t10422: F, t10393: F, t3070: F, t11046: F, t42387: F, t10457: F, t820: F, t10409: F, t10936: F, t3180: F, t10390: F, t10394: F, t10398: F, t1041: F, t10428: F, t10433: F, t10884: F, t10891: F, t10904: F, t10915: F, t10919: F, t10932: F, t14187: F, t2960: F, t3048: F, t3071: F, t3073: F, t42460: F, t42468: F, t4582: F, t884: F, t10401: F, t10935: F, t3186: F, t3200: F, t11051: F, t3069: F, t10454: F, t10459: F, t3036: F, t3087: F, t3033: F, t3128: F, t10987: F, t135: F, t973: F, t10405: F, t10408: F, t10415: F, t10937: F, t10944: F, t10957: F, t10988: F, t2771: F, t2780: F, t3064: F, t3121: F, t3134: F, t10402: F, t11034: F, t11037: F, t2402: F, t999: F, t9277: F, t972: F, t10263: F, t3139: F, t1030: F, t10477: F, t10472: F, t10475: F, t10903: F, t10948: F, t10890: F, t10898: F, t3103: F, t1000: F, t10410: F, t10485: F, t10860: F, t10879: F, t3043: F, t3109: F, t3117: F, t3123: F, t11002: F, t10508: F, t248: F, t3130: F, t3132: F, t10969: F, t121: F, t10305: F, t1015: F, t3142: F, t698: F, t3147: F, t10981: F, t1044: F, t10972: F, t3057: F, t3098: F, t3114: F, t3143: F, t3148: F, t41709: F, t10984: F, t10213: F, t41687: F, t10857: F, t376: F, t1004: F, t10956: F, t10863: F, t3053: F, t10516: F, t3113: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42478, t42481, t42483, t42490, t42496) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1314::<F>(t10413, t10414, t10422, t10393, t3070, t11046, t42387, t10457, t820, t10409, t10936, t3180);
        let t42499 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1315::<F>(t10390, t10394, t10398, t1041, t10428, t10433, t10884, t10891, t10904, t10915, t10919, t10932, t14187, t2960, t3048, t3071, t3073, t42460, t42468, t42478, t42481, t42483, t42490, t42496, t4582, t884);
        let (t42505, t42508, t42511, t42514, t42518, t42520, t42522) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1316::<F>(t10401, t10935, t3186, t3200, t11051, t3069, t10454, t3048, t10459, t3036, t3087, t3033, t3128);
        let t42540 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1317::<F>(t10987, t135, t973, t10394, t10405, t10408, t10415, t10937, t10944, t10957, t10988, t2771, t2780, t2960, t3064, t3070, t3071, t3073, t3121, t3134, t42505, t42508, t42511, t42514, t42518, t42522);
        let (t42541, t42546, t42552, t42554, t42557, t42559) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1318::<F>(t10402, t11034, t11037, t2402, t973, t999, t9277, t972, t10263, t3139, t1030, t10477);
        let t42580 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319::<F>(t10472, t10475, t42559, t3128, t10903, t10948, t10890, t10898, t3103, t1000, t10390, t10405, t10410, t10415, t10485, t10860, t10879, t10919, t3043, t3109, t3117, t3123, t3134, t42541, t42546, t42552, t42554, t42557);
        let (t42582, t42586, t42595, t42600) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1320::<F>(t10904, t11002, t10508, t248, t3130, t3132, t10969, t121, t10305, t1041, t1015, t3033, t42520);
        let t42621 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1321::<F>(t3142, t698, t973, t3147, t10981, t2960, t10263, t1041, t1044, t10860, t10957, t10972, t248, t3043, t3048, t3057, t3098, t3114, t3143, t3148, t41709, t42582, t42586, t42595, t42600);
        let (t42622, t42624, t42639, t42648, t42651, t42653) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1322::<F>(t10984, t2960, t10213, t41687, t10857, t376, t1004, t10956, t10863, t3053, t10516, t3113);
    (t42499, t42540, t42554, t42559, t42580, t42621, t42622, t42624, t42639, t42648, t42651, t42653)
}
