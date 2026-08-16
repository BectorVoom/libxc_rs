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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta672(t50946: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64, t50957: f64, t50961: f64, t50966: f64, t50968: f64, t50970: f64, t50972: f64, t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t44053: f64, t50976: f64, t50978: f64, t50987: f64, t50990: f64, t50994: f64, t51000: f64, t51004: f64, t51007: f64, t51010: f64, t51012: f64, t51014: f64, t51016: f64, t51018: f64, t51021: f64, t51024: f64, t51027: f64, t51030: f64, t51034: f64, t51037: f64, t51039: f64, t51041: f64, t51043: f64, t51046: f64, t51049: f64, t51051: f64, t51053: f64, t51056: f64, t51100: f64, t51102: f64, t1099: f64, t1118: f64, t51147: f64, t51159: f64, t51173: f64, t51186: f64, t3263: f64, t4737: f64, t3266: f64, t11189: f64, t1657: f64, t11192: f64, t50826: f64, t43727: f64, t43729: f64, t43748: f64, t43750: f64, t50824: f64, t50828: f64, t50832: f64, t50834: f64, t50837: f64, t50839: f64, t50853: f64, t43768: f64, t43770: f64, t44249: f64, t50846: f64, t50848: f64, t50851: f64, t50859: f64, t50863: f64, t50867: f64, t50871: f64, t50875: f64, t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t50881: f64, t50886: f64, t50897: f64, t50900: f64, t50919: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64, t50917: f64, t50921: f64, t50926: f64, t50931: f64, t50934: f64, t50937: f64, t50940: f64, t44275: f64, t1147: f64, t14933: f64, t3400: f64, t4832: f64, t11282: f64, t1687: f64, t1129: f64, t11311: f64, t1137: f64, t11400: f64, t11410: f64, t1157: f64, t15118: f64, t15121: f64, t1695: f64, t3327: f64, t3396: f64, t3404: f64, t44183: f64, t4820: f64, t4835: f64, t50821: f64, t51119: f64, t51122: f64, t51124: f64, t51126: f64, t51128: f64) -> (f64, f64, f64, f64) {
        let (t51200, t51212) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2526(t50946, t50948, t50950, t50952, t50954, t50957, t50961, t50966, t50968, t50970, t50972, t43780, t43782, t43784, t43786, t43788, t43816, t44053, t50976, t50978, t50987, t50990, t50994);
        let t51226 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2527(t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021, t51024, t51027, t51030);
        let t51239 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2528(t51034, t51037, t51039, t51041, t51043, t51046, t51049, t51051, t51053, t51056, t51100, t51102);
        let (t51245, t51246) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2529(t1099, t1118, t51147, t51159, t51173, t51186, t51200, t51212, t51226, t51239, t3263, t4737);
        let (t51248, t51251, t51267) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2530(t3266, t51246, t11189, t1657, t11192, t50826, t43727, t43729, t43748, t43750, t50824, t50828, t50832, t50834, t50837, t50839);
        let t51279 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2531(t50853, t43768, t43770, t44249, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875);
        let t51293 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2532(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t51306 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2533(t50919, t50903, t50905, t50907, t50912, t50917, t50921, t50926, t50931, t50934, t50937, t50940);
        let (t51320, t51332) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2534(t50948, t50946, t50950, t50952, t50954, t50957, t50961, t50966, t50968, t50970, t50972, t43780, t43782, t43784, t43786, t43788, t43816, t44275, t50976, t50978, t50987, t50990, t50994);
        let t51346 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2535(t51000, t51004, t51007, t51010, t51012, t51014, t51016, t51018, t51021, t51024, t51027, t51030);
        let t51359 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2536(t51039, t51051, t51034, t51037, t51041, t51043, t51046, t51049, t51053, t51056, t51100, t51102);
        let t51381 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2537(t1147, t14933, t3400, t4832, t11282, t1687, t1129, t11311, t1137, t11400, t11410, t1157, t15118, t15121, t1695, t3327, t3396, t3404, t44183, t4820, t4835, t50821, t51119, t51122, t51124, t51126, t51128, t51267, t51279, t51293, t51306, t51320, t51332, t51346, t51359);
    (t51245, t51248, t51251, t51381)
}
