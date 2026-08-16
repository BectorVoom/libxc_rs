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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta367(t41961: f64, t41845: f64, t41863: f64, t41865: f64, t41868: f64, t41870: f64, t41872: f64, t41874: f64, t41876: f64, t41882: f64, t41885: f64, t41973: f64, t3008: f64, t10199: f64, t2970: f64, t973: f64, t10200: f64, t10214: f64, t10219: f64, t10235: f64, t10278: f64, t2960: f64, t2986: f64, t340: f64, t343: f64, t39097: f64, t39110: f64, t42968: f64, t42974: f64, t42976: f64, t42985: f64, t43000: f64, t974: f64, t977: f64, t978: f64, t10203: f64, t10254: f64, t10913: f64, t697: f64, t976: f64, t984: f64, t2990: f64, t10189: f64, t10325: f64, t2987: f64, t4509: f64, t13797: f64, t10216: f64, t9288: f64, t10236: f64, t10186: f64, t10204: f64, t10237: f64, t10241: f64, t10245: f64, t10251: f64, t10259: f64, t13831: f64, t2988: f64, t42790: f64, t42824: f64, t42860: f64, t42899: f64, t42933: f64, t42966: f64, t225: f64, t10427: f64, t13969: f64, t3130: f64, t10432: f64, t3039: f64, t1021: f64, t1025: f64, t1041: f64, t1044: f64, t1046: f64, t10863: f64, t248: f64, t3043: f64, t3064: f64, t3131: f64, t369: f64, t378: f64, t41671: f64, t42422: f64, t42729: f64, t42731: f64, t42735: f64, t42743: f64, t42746: f64, t42752: f64, t42756: f64, t68: f64, t10943: f64, t135: f64, t3152: f64, t698: f64, t10870: f64, t3117: f64, t1020: f64, t10858: f64, t3101: f64, t10961: f64, t3108: f64, t10403: f64, t10426: f64, t10428: f64, t10480: f64, t10501: f64, t10517: f64, t10915: f64, t10949: f64, t10965: f64, t13980: f64, t13985: f64, t14213: f64, t3071: f64, t3098: f64, t3123: f64, t42639: f64, t4582: f64, t4594: f64, t998: f64, t10423: f64, t10937: f64, t2955: f64, t3158: f64, t10383: f64, t964: f64, t10508: f64, t3121: f64, t11002: f64, t1036: f64, t10361: f64, t1031: f64, t10360: f64, t10413: f64, t10419: f64, t10970: f64, t2780: f64, t3041: f64, t3077: f64, t3088: f64, t3132: f64, t41640: f64, t41688: f64, t10390: f64, t10868: f64, t820: f64, t3070: f64, t3072: f64, t10489: f64, t1015: f64, t10472: f64, t42559: f64, t3048: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t43012 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1344(t41961, t41845, t41863, t41865, t41868, t41870, t41872, t41874, t41876, t41882, t41885, t41973);
        let t43034 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1345(t3008, t10199, t2970, t973, t10200, t10214, t10219, t10235, t10278, t2960, t2986, t340, t343, t39097, t39110, t42968, t42974, t42976, t42985, t43000, t43012, t974, t977, t978);
        let (t43038, t43043, t43055, t43057) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1346(t10203, t2970, t973, t10254, t10913, t697, t976, t984, t2986, t2990, t10189, t3008);
        let (t43059, t43061, t43065, t43069, t43071) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1347(t2986, t2990, t43057, t10325, t2987, t3008, t4509, t13797, t984, t10216, t343, t9288);
        let t43079 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1348(t10236, t9288, t10186, t10204, t10237, t10241, t10245, t10251, t10259, t13831, t2960, t2986, t2988, t2990, t43038, t43043, t43055, t43059, t43061, t43065, t43069, t43071);
        let (t43082, t43083, t43094) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349(t42790, t42824, t42860, t42899, t42933, t42966, t43034, t43079, t225, t10427, t13969, t3130);
        let t43099 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1350(t10432, t13969, t3039, t1021, t1025, t1041, t1044, t1046, t10863, t248, t3043, t3064, t3130, t3131, t369, t378, t41671, t42422, t42729, t42731, t42735, t42743, t42746, t42752, t42756, t43083, t43094, t68);
        let (t43103, t43110, t43114, t43118, t43120) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1351(t10943, t135, t973, t3152, t698, t10870, t3117, t1020, t10858, t248, t3101, t10961, t3108);
        let t43141 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1352(t1025, t10403, t10426, t10428, t10480, t10501, t10517, t10915, t10949, t10965, t13980, t13985, t14213, t3071, t3098, t3117, t3123, t3130, t39110, t42639, t43103, t43110, t43114, t43118, t43120, t4582, t4594, t973, t974, t998);
        let (t43143, t43155, t43157, t43161, t43167) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1353(t10423, t10937, t2955, t3158, t10383, t964, t1020, t10508, t248, t3121, t10949, t11002);
        let t43181 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1354(t1036, t10361, t1031, t10360, t10403, t1041, t10413, t10419, t1044, t10937, t10970, t248, t2780, t3041, t3071, t3077, t3088, t3132, t378, t41640, t41688, t43143, t43155, t43157, t43161, t43167);
        let (t43186, t43200, t43206, t43211, t43214) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1355(t10390, t10423, t10868, t820, t3070, t3072, t10489, t3117, t1015, t10472, t42559, t10870, t3048);
    (t43082, t43083, t43099, t43141, t43181, t43186, t43200, t43206, t43211, t43214)
}
