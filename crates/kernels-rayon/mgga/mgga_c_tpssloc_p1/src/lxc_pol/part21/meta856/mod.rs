//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta856 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3096;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3098;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3099;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3101;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta856(t43748: f64, t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63891: f64, t63893: f64, t63896: f64, t63899: f64, t63903: f64, t63906: f64, t63909: f64, t63911: f64, t63914: f64, t50846: f64, t50848: f64, t50853: f64, t63918: f64, t63921: f64, t63924: f64, t63927: f64, t63930: f64, t63933: f64, t63936: f64, t63939: f64, t63997: f64, t64003: f64, t64006: f64, t64009: f64, t43855: f64, t43859: f64, t43861: f64, t43863: f64, t44249: f64, t50903: f64, t50905: f64, t50907: f64, t50919: f64, t50921: f64, t50948: f64, t50950: f64, t50952: f64, t50954: f64, t43780: f64, t43782: f64, t43816: f64, t44275: f64, t50968: f64, t50970: f64, t50972: f64, t50978: f64, t51039: f64, t51041: f64, t64028: f64, t64031: f64, t64033: f64, t64042: f64, t64045: f64, t51043: f64, t51051: f64, t51053: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64, t63398: f64, t63400: f64, t63404: f64, t63408: f64, t63412: f64, t63417: f64, t63422: f64, t64074: f64, t64076: f64, t64079: f64, t64082: f64, t64085: f64, t64087: f64, t64089: f64, t64092: f64, t43959: f64, t6024: f64, t1128: f64, t18668: f64, t3263: f64, t5983: f64, t3266: f64, t1129: f64, t1137: f64, t1138: f64, t11410: f64, t15118: f64, t15141: f64, t1683: f64, t18840: f64, t18894: f64, t3327: f64, t3352: f64, t3360: f64, t44211: f64, t4797: f64, t4820: f64, t51594: f64, t6037: f64, t6053: f64, t63763: f64, t63765: f64, t63767: f64, t63769: f64, t63771: f64, t63829: f64, t64100: f64, t64103: f64, t64132: f64, t64148: f64, t4819: f64, t3331: f64, t6031: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63841: f64, t63843: f64, t63845: f64, t63323: f64, t63327: f64, t63330: f64, t63848: f64, t63853: f64, t63856: f64, t63858: f64, t63860: f64, t63862: f64, t63865: f64, t63867: f64, t63870: f64, t63873: f64, t63876: f64, t63879: f64, t43777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t64165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3096(t43748, t63332, t63334, t63336, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909, t63911, t63914);
        let t64181 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3097(t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63997, t64003, t64006, t64009);
        let t64197 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3098(t43855, t43859, t43861, t43863, t44249, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
        let t64212 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3099(t43780, t43782, t43816, t44275, t50968, t50970, t50972, t50978, t51039, t51041, t64028, t64031, t64033, t64042, t64045);
        let t64229 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100(t51043, t51051, t51053, t63355, t63359, t63361, t63365, t63370, t63374, t63380, t63382, t63384, t63388, t63392, t63396);
        let t64245 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3101(t63398, t63400, t63404, t63408, t63412, t63417, t63422, t64074, t64076, t64079, t64082, t64085, t64087, t64089, t64092);
        let (t64253, t64259, t64260) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102(t43959, t6024, t1128, t18668, t3263, t5983, t3266, t1129, t1137, t1138, t11410, t15118, t15141, t1683, t18840, t18894, t3327, t3352, t3360, t44211, t4797, t4820, t51594, t6037, t6053, t63763, t63765, t63767, t63769, t63771, t63829, t64100, t64103, t64132, t64148, t64165, t64181, t64197, t64212, t64229, t64245);
        let (t64261, t64292, t64309) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103(t4819, t3331, t6031, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63841, t63843, t63845);
        let t64325 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3104(t63323, t63327, t63330, t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867, t63870, t63873, t63876, t63879);
        let t64342 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105(t43748, t63332, t63334, t63336, t63886, t63888, t63891, t63893, t63896, t63899, t63903, t63906, t63909, t63911, t63914);
        let t64358 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3106(t50846, t50848, t50853, t63918, t63921, t63924, t63927, t63930, t63933, t63936, t63939, t63997, t64003, t64006, t64009);
        let t64374 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3107(t43777, t43855, t43859, t43861, t43863, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t50952, t50954);
    (t64253, t64259, t64260, t64261, t64292, t64309, t64325, t64342, t64358, t64374)
}
