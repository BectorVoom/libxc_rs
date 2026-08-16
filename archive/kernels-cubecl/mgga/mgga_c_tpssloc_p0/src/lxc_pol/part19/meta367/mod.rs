//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta367 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1344;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1345;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1346;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1347;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1348;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1350;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1351;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352;
use chunk9::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1353;
use chunk10::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1354;
use chunk11::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta367<F: Float>(t41961: F, t41845: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41882: F, t41885: F, t41973: F, t3008: F, t10199: F, t2970: F, t973: F, t10200: F, t10214: F, t10219: F, t10235: F, t10278: F, t2960: F, t2986: F, t340: F, t343: F, t39097: F, t39110: F, t42968: F, t42974: F, t42976: F, t42985: F, t43000: F, t974: F, t977: F, t978: F, t10203: F, t10254: F, t10913: F, t697: F, t976: F, t984: F, t2990: F, t10189: F, t10325: F, t2987: F, t4509: F, t13797: F, t10216: F, t9288: F, t10236: F, t10186: F, t10204: F, t10237: F, t10241: F, t10245: F, t10251: F, t10259: F, t13831: F, t2988: F, t42790: F, t42824: F, t42860: F, t42899: F, t42933: F, t42966: F, t225: F, t10427: F, t13969: F, t3130: F, t10432: F, t3039: F, t1021: F, t1025: F, t1041: F, t1044: F, t1046: F, t10863: F, t248: F, t3043: F, t3064: F, t3131: F, t369: F, t378: F, t41671: F, t42422: F, t42729: F, t42731: F, t42735: F, t42743: F, t42746: F, t42752: F, t42756: F, t68: F, t10943: F, t135: F, t3152: F, t698: F, t10870: F, t3117: F, t1020: F, t10858: F, t3101: F, t10961: F, t3108: F, t10403: F, t10426: F, t10428: F, t10480: F, t10501: F, t10517: F, t10915: F, t10949: F, t10965: F, t13980: F, t13985: F, t14213: F, t3071: F, t3098: F, t3123: F, t42639: F, t4582: F, t4594: F, t998: F, t10423: F, t10937: F, t2955: F, t3158: F, t10383: F, t964: F, t10508: F, t3121: F, t11002: F, t1036: F, t10361: F, t1031: F, t10360: F, t10413: F, t10419: F, t10970: F, t2780: F, t3041: F, t3077: F, t3088: F, t3132: F, t41640: F, t41688: F, t10390: F, t10868: F, t820: F, t3070: F, t3072: F, t10489: F, t1015: F, t10472: F, t42559: F, t3048: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t43012 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1344::<F>(t41961, t41845, t41863, t41865, t41868, t41870, t41872, t41874, t41876, t41882, t41885, t41973);
        let t43034 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1345::<F>(t3008, t10199, t2970, t973, t10200, t10214, t10219, t10235, t10278, t2960, t2986, t340, t343, t39097, t39110, t42968, t42974, t42976, t42985, t43000, t43012, t974, t977, t978);
        let (t43038, t43043, t43055, t43057) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1346::<F>(t10203, t2970, t973, t10254, t10913, t697, t976, t984, t2986, t2990, t10189, t3008);
        let (t43059, t43061, t43065, t43069, t43071) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1347::<F>(t2986, t2990, t43057, t10325, t2987, t3008, t4509, t13797, t984, t10216, t343, t9288);
        let t43079 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1348::<F>(t10236, t9288, t10186, t10204, t10237, t10241, t10245, t10251, t10259, t13831, t2960, t2986, t2988, t2990, t43038, t43043, t43055, t43059, t43061, t43065, t43069, t43071);
        let (t43082, t43083, t43094) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349::<F>(t42790, t42824, t42860, t42899, t42933, t42966, t43034, t43079, t225, t10427, t13969, t3130);
        let t43099 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1350::<F>(t10432, t13969, t3039, t1021, t1025, t1041, t1044, t1046, t10863, t248, t3043, t3064, t3130, t3131, t369, t378, t41671, t42422, t42729, t42731, t42735, t42743, t42746, t42752, t42756, t43083, t43094, t68);
        let (t43103, t43110, t43114, t43118, t43120) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1351::<F>(t10943, t135, t973, t3152, t698, t10870, t3117, t1020, t10858, t248, t3101, t10961, t3108);
        let t43141 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352::<F>(t1025, t10403, t10426, t10428, t10480, t10501, t10517, t10915, t10949, t10965, t13980, t13985, t14213, t3071, t3098, t3117, t3123, t3130, t39110, t42639, t43103, t43110, t43114, t43118, t43120, t4582, t4594, t973, t974, t998);
        let (t43143, t43155, t43157, t43161, t43167) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1353::<F>(t10423, t10937, t2955, t3158, t10383, t964, t1020, t10508, t248, t3121, t10949, t11002);
        let t43181 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1354::<F>(t1036, t10361, t1031, t10360, t10403, t1041, t10413, t10419, t1044, t10937, t10970, t248, t2780, t3041, t3071, t3077, t3088, t3132, t378, t41640, t41688, t43143, t43155, t43157, t43161, t43167);
        let (t43186, t43200, t43206, t43211, t43214) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1355::<F>(t10390, t10423, t10868, t820, t3070, t3072, t10489, t3117, t1015, t10472, t42559, t10870, t3048);
    (t43082, t43083, t43099, t43141, t43181, t43186, t43200, t43206, t43211, t43214)
}
