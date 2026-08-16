//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta658 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2440;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2441;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2442;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2443;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2444;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2445;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2446;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2447;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2448;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2450;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2451;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta658(t14137: f64, t3048: f64, t10952: f64, t13970: f64, t13969: f64, t14098: f64, t3039: f64, t10224: f64, t4343: f64, t973: f64, t3130: f64, t4595: f64, t49850: f64, t10408: f64, t10428: f64, t10919: f64, t14152: f64, t14508: f64, t1618: f64, t2771: f64, t2960: f64, t3070: f64, t42573: f64, t42658: f64, t43103: f64, t43110: f64, t4600: f64, t4644: f64, t4650: f64, t47746: f64, t977: f64, t10402: f64, t14618: f64, t14608: f64, t13981: f64, t10422: f64, t14129: f64, t11002: f64, t10895: f64, t14511: f64, t10405: f64, t10410: f64, t10415: f64, t10863: f64, t10904: f64, t10937: f64, t13541: f64, t13982: f64, t13995: f64, t14130: f64, t14143: f64, t14147: f64, t14228: f64, t3071: f64, t4585: f64, t14207: f64, t3103: f64, t14085: f64, t3053: f64, t14080: f64, t1022: f64, t2244: f64, t360: f64, t10936: f64, t4669: f64, t14077: f64, t1036: f64, t14114: f64, t10390: f64, t10860: f64, t13536: f64, t14235: f64, t1622: f64, t3073: f64, t42397: f64, t42648: f64, t43114: f64, t43118: f64, t43298: f64, t4641: f64, t3082: f64, t4617: f64, t3132: f64, t607: f64, t3120: f64, t4594: f64, t14025: f64, t10403: f64, t1041: f64, t13975: f64, t13980: f64, t13991: f64, t14009: f64, t14230: f64, t1539: f64, t42334: f64, t42522: f64, t43241: f64, t4337: f64, t4342: f64, t4582: f64, t4583: f64, t4596: f64, t45997: f64, t48506: f64, t4584: f64, t14032: f64, t14166: f64, t1023: f64, t10483: f64, t13611: f64, t13762: f64, t14012: f64, t14189: f64, t2979: f64, t42388: f64, t43143: f64, t43155: f64, t43157: f64, t43161: f64, t47726: f64, t14159: f64, t14146: f64, t14068: f64, t10263: f64, t4603: f64, t10891: f64, t10250: f64, t10884: f64, t14172: f64, t14184: f64, t1607: f64, t1616: f64, t3117: f64, t42358: f64, t42554: f64, t42756: f64, t43167: f64, t4593: f64, t48554: f64, t10231: f64, t13528: f64, t13532: f64, t13537: f64, t42972: f64, t135: f64, t14197: f64, t14015: f64, t14018: f64, t14174: f64, t14180: f64, t14198: f64, t4590: f64, t47684: f64, t47759: f64, t47763: f64, t43198: f64, t4578: f64, t4574: f64, t14192: f64, t10510: f64, t10316: f64, t10481: f64, t10877: f64, t14099: f64, t42347: f64, t42511: f64, t42743: f64, t43176: f64, t43291: f64, t43292: f64, t43385: f64, t4579: f64, t45872: f64, t974: f64, t998: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49892, t49894, t49897, t49907, t49922) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2440(t14137, t3048, t10952, t13970, t13969, t14098, t3039, t10224, t4343, t973, t3130, t4595, t49850);
        let t49924 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2441(t49922, t10408, t10428, t10919, t14152, t14508, t1618, t2771, t2960, t3070, t42573, t42658, t43103, t43110, t4600, t4644, t4650, t47746, t49892, t49894, t49897, t49907, t973, t977);
        let (t49929, t49934, t49940, t49945, t49957, t49959) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2442(t10402, t14618, t14608, t13969, t13981, t3130, t10422, t14129, t3070, t11002, t14508, t10895, t14511);
        let t49961 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2443(t10405, t10410, t10415, t10863, t10904, t10937, t13541, t13982, t13995, t14130, t14143, t14147, t14228, t3048, t3070, t3071, t4585, t49929, t49934, t49940, t49945, t49957, t49959);
        let (t49964, t49966, t49972, t49976, t49984, t49987, t49989) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2444(t14207, t3103, t14085, t3053, t14080, t1022, t2244, t360, t10936, t4669, t14077, t1036, t14114);
        let t49991 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2445(t10390, t10860, t13536, t14235, t1622, t3070, t3073, t42397, t42648, t43114, t43118, t43298, t4641, t49964, t49966, t49972, t49976, t49984, t49987, t49989);
        let (t50014, t50035) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2446(t3082, t4617, t3132, t607, t3120, t4594, t10904, t14025, t10403, t10408, t1041, t10937, t13975, t13980, t13991, t14009, t14230, t1539, t2960, t3070, t3071, t3130, t42334, t42522, t43241, t4337, t4342, t4582, t4583, t4596, t45997, t48506);
        let t50066 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2447(t1041, t4584, t49850, t10422, t14032, t3070, t13969, t14166, t1023, t10390, t10483, t13611, t13762, t14012, t14189, t1539, t2960, t2979, t3048, t3071, t42388, t43143, t43155, t43157, t43161, t47726, t973);
        let (t50078, t50084, t50094, t50098, t50100) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2448(t14159, t2960, t1041, t13969, t14146, t10422, t14068, t3070, t10263, t4603, t10891, t13970);
        let t50102 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449(t10250, t1041, t10884, t14172, t14184, t1607, t1616, t1618, t3048, t3070, t3071, t3117, t42358, t42554, t42756, t43167, t4582, t4593, t48554, t50078, t50084, t50094, t50098, t50100);
        let t50136 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2450(t10231, t13528, t973, t13532, t13537, t42972, t135, t14197, t10863, t14015, t14018, t14174, t14180, t14198, t2960, t2979, t3048, t4590, t47684, t47759, t47763, t977);
        let t50176 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2451(t3070, t43198, t4578, t4574, t14192, t2960, t10510, t4641, t10316, t10481, t10483, t10877, t10952, t14099, t1616, t3071, t42347, t42511, t42743, t43176, t43291, t43292, t43385, t4579, t4582, t45872, t4593, t4600, t973, t974, t998);
    (t49924, t49961, t49976, t49991, t50014, t50035, t50066, t50102, t50136, t50176)
}
