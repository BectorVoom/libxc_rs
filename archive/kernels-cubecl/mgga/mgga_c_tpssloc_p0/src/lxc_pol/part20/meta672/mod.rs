//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta672 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2526;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2527;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2528;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2529;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2530;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2531;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2532;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2533;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2534;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2535;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2536;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta672<F: Float>(t50946: F, t50948: F, t50950: F, t50952: F, t50954: F, t50957: F, t50961: F, t50966: F, t50968: F, t50970: F, t50972: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43816: F, t44053: F, t50976: F, t50978: F, t50987: F, t50990: F, t50994: F, t51000: F, t51004: F, t51007: F, t51010: F, t51012: F, t51014: F, t51016: F, t51018: F, t51021: F, t51024: F, t51027: F, t51030: F, t51034: F, t51037: F, t51039: F, t51041: F, t51043: F, t51046: F, t51049: F, t51051: F, t51053: F, t51056: F, t51100: F, t51102: F, t1099: F, t1118: F, t51147: F, t51159: F, t51173: F, t51186: F, t3263: F, t4737: F, t3266: F, t11189: F, t1657: F, t11192: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50828: F, t50832: F, t50834: F, t50837: F, t50839: F, t50853: F, t43768: F, t43770: F, t44249: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t50881: F, t50886: F, t50897: F, t50900: F, t50919: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F, t50937: F, t50940: F, t44275: F, t1147: F, t14933: F, t3400: F, t4832: F, t11282: F, t1687: F, t1129: F, t11311: F, t1137: F, t11400: F, t11410: F, t1157: F, t15118: F, t15121: F, t1695: F, t3327: F, t3396: F, t3404: F, t44183: F, t4820: F, t4835: F, t50821: F, t51119: F, t51122: F, t51124: F, t51126: F, t51128: F) -> (F, F, F, F) {
        let (t51200, t51212) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2526::<F>(t50946, t50948, t50950, t50952, t50954, t50957, t50961, t50966, t50968, t50970, t50972, t43780, t43782, t43784, t43786, t43788, t43816, t44053, t50976, t50978, t50987, t50990, t50994);
        let t51226 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2527::<F>(t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021, t51024, t51027, t51030);
        let t51239 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2528::<F>(t51034, t51037, t51039, t51041, t51043, t51046, t51049, t51051, t51053, t51056, t51100, t51102);
        let (t51245, t51246) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2529::<F>(t1099, t1118, t51147, t51159, t51173, t51186, t51200, t51212, t51226, t51239, t3263, t4737);
        let (t51248, t51251, t51267) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2530::<F>(t3266, t51246, t11189, t1657, t11192, t50826, t43727, t43729, t43748, t43750, t50824, t50828, t50832, t50834, t50837, t50839);
        let t51279 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2531::<F>(t50853, t43768, t43770, t44249, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875);
        let t51293 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2532::<F>(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t51306 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2533::<F>(t50919, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934, t50937, t50940);
        let (t51320, t51332) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2534::<F>(t50948, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50968, t50970, t50972, t43780, t43782, t43784, t43786, t43788, t43816, t44275, t50976, t50978, t50987, t50990, t50994);
        let t51346 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2535::<F>(t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021, t51024, t51027, t51030);
        let t51359 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2536::<F>(t51039, t51051, t51034, t51037, t51041, t51043, t51046, t51049, t51053, t51056, t51100, t51102);
        let t51381 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2537::<F>(t1147, t14933, t3400, t4832, t11282, t1687, t1129, t11311, t1137, t11400, t11410, t1157, t15118, t15121, t1695, t3327, t3396, t3404, t44183, t4820, t4835, t50821, t51119, t51122, t51124, t51126, t51128, t51267, t51279, t51293, t51306, t51320, t51332, t51346, t51359);
    (t51245, t51248, t51251, t51381)
}
