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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

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
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta658<F: Float>(t14137: F, t3048: F, t10952: F, t13970: F, t13969: F, t14098: F, t3039: F, t10224: F, t4343: F, t973: F, t3130: F, t4595: F, t49850: F, t10408: F, t10428: F, t10919: F, t14152: F, t14508: F, t1618: F, t2771: F, t2960: F, t3070: F, t42573: F, t42658: F, t43103: F, t43110: F, t4600: F, t4644: F, t4650: F, t47746: F, t977: F, t10402: F, t14618: F, t14608: F, t13981: F, t10422: F, t14129: F, t11002: F, t10895: F, t14511: F, t10405: F, t10410: F, t10415: F, t10863: F, t10904: F, t10937: F, t13541: F, t13982: F, t13995: F, t14130: F, t14143: F, t14147: F, t14228: F, t3071: F, t4585: F, t14207: F, t3103: F, t14085: F, t3053: F, t14080: F, t1022: F, t2244: F, t360: F, t10936: F, t4669: F, t14077: F, t1036: F, t14114: F, t10390: F, t10860: F, t13536: F, t14235: F, t1622: F, t3073: F, t42397: F, t42648: F, t43114: F, t43118: F, t43298: F, t4641: F, t3082: F, t4617: F, t3132: F, t607: F, t3120: F, t4594: F, t14025: F, t10403: F, t1041: F, t13975: F, t13980: F, t13991: F, t14009: F, t14230: F, t1539: F, t42334: F, t42522: F, t43241: F, t4337: F, t4342: F, t4582: F, t4583: F, t4596: F, t45997: F, t48506: F, t4584: F, t14032: F, t14166: F, t1023: F, t10483: F, t13611: F, t13762: F, t14012: F, t14189: F, t2979: F, t42388: F, t43143: F, t43155: F, t43157: F, t43161: F, t47726: F, t14159: F, t14146: F, t14068: F, t10263: F, t4603: F, t10891: F, t10250: F, t10884: F, t14172: F, t14184: F, t1607: F, t1616: F, t3117: F, t42358: F, t42554: F, t42756: F, t43167: F, t4593: F, t48554: F, t10231: F, t13528: F, t13532: F, t13537: F, t42972: F, t135: F, t14197: F, t14015: F, t14018: F, t14174: F, t14180: F, t14198: F, t4590: F, t47684: F, t47759: F, t47763: F, t43198: F, t4578: F, t4574: F, t14192: F, t10510: F, t10316: F, t10481: F, t10877: F, t14099: F, t42347: F, t42511: F, t42743: F, t43176: F, t43291: F, t43292: F, t43385: F, t4579: F, t45872: F, t974: F, t998: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49892, t49894, t49897, t49907, t49922) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2440::<F>(t14137, t3048, t10952, t13970, t13969, t14098, t3039, t10224, t4343, t973, t3130, t4595, t49850);
        let t49924 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2441::<F>(t49922, t10408, t10428, t10919, t14152, t14508, t1618, t2771, t2960, t3070, t42573, t42658, t43103, t43110, t4600, t4644, t4650, t47746, t49892, t49894, t49897, t49907, t973, t977);
        let (t49929, t49934, t49940, t49945, t49957, t49959) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2442::<F>(t10402, t14618, t14608, t13969, t13981, t3130, t10422, t14129, t3070, t11002, t14508, t10895, t14511);
        let t49961 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2443::<F>(t10405, t10410, t10415, t10863, t10904, t10937, t13541, t13982, t13995, t14130, t14143, t14147, t14228, t3048, t3070, t3071, t4585, t49929, t49934, t49940, t49945, t49957, t49959);
        let (t49964, t49966, t49972, t49976, t49984, t49987, t49989) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2444::<F>(t14207, t3103, t14085, t3053, t14080, t1022, t2244, t360, t10936, t4669, t14077, t1036, t14114);
        let t49991 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2445::<F>(t10390, t10860, t13536, t14235, t1622, t3070, t3073, t42397, t42648, t43114, t43118, t43298, t4641, t49964, t49966, t49972, t49976, t49984, t49987, t49989);
        let (t50014, t50035) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2446::<F>(t3082, t4617, t3132, t607, t3120, t4594, t10904, t14025, t10403, t10408, t1041, t10937, t13975, t13980, t13991, t14009, t14230, t1539, t2960, t3070, t3071, t3130, t42334, t42522, t43241, t4337, t4342, t4582, t4583, t4596, t45997, t48506);
        let t50066 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2447::<F>(t1041, t4584, t49850, t10422, t14032, t3070, t13969, t14166, t1023, t10390, t10483, t13611, t13762, t14012, t14189, t1539, t2960, t2979, t3048, t3071, t42388, t43143, t43155, t43157, t43161, t47726, t973);
        let (t50078, t50084, t50094, t50098, t50100) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2448::<F>(t14159, t2960, t1041, t13969, t14146, t10422, t14068, t3070, t10263, t4603, t10891, t13970);
        let t50102 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2449::<F>(t10250, t1041, t10884, t14172, t14184, t1607, t1616, t1618, t3048, t3070, t3071, t3117, t42358, t42554, t42756, t43167, t4582, t4593, t48554, t50078, t50084, t50094, t50098, t50100);
        let t50136 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2450::<F>(t10231, t13528, t973, t13532, t13537, t42972, t135, t14197, t10863, t14015, t14018, t14174, t14180, t14198, t2960, t2979, t3048, t4590, t47684, t47759, t47763, t977);
        let t50176 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2451::<F>(t3070, t43198, t4578, t4574, t14192, t2960, t10510, t4641, t10316, t10481, t10483, t10877, t10952, t14099, t1616, t3071, t42347, t42511, t42743, t43176, t43291, t43292, t43385, t4579, t4582, t45872, t4593, t4600, t973, t974, t998);
    (t49924, t49961, t49976, t49991, t50014, t50035, t50066, t50102, t50136, t50176)
}
