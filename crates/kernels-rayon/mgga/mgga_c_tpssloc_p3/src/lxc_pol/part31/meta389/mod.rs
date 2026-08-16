//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1388;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1389;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1390;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1391;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1392;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta389(t14164: f64, t17686: f64, t4582: f64, t17691: f64, t4583: f64, t1023: f64, t17670: f64, t4594: f64, t17167: f64, t977: f64, t17171: f64, t17157: f64, t2979: f64, t5677: f64, t10408: f64, t1036: f64, t5905: f64, t1041: f64, t10876: f64, t10883: f64, t10952: f64, t13995: f64, t14158: f64, t14160: f64, t3070: f64, t3109: f64, t4579: f64, t5869: f64, t5880: f64, t973: f64, t4571: f64, t4644: f64, t1031: f64, t5904: f64, t1022: f64, t1539: f64, t14211: f64, t3071: f64, t5685: f64, t1616: f64, t4343: f64, t1009: f64, t5848: f64, t1011: f64, t1019: f64, t5873: f64, t884: f64, t10422: f64, t5908: f64, t1025: f64, t10403: f64, t10923: f64, t10937: f64, t14194: f64, t14203: f64, t14495: f64, t14503: f64, t3117: f64, t378: f64, t5900: f64, t5909: f64, t17614: f64, t17640: f64, t17684: f64, t17725: f64, t17900: f64, t17967: f64, t349: f64, t1052: f64, t1066: f64, t17575: f64, t17579: f64, t17583: f64, t17588: f64, t3026: f64, t3169: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t5920: f64, t5944: f64, t5914: f64, t990: f64, t17875: f64, t381: f64, t1049: f64, t1065: f64, t5943: f64, t3174: f64, t1625: f64, t4552: f64, t5919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17972, t17976, t17980, t17984, t17988, t17991, t17994) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1388(t14164, t17686, t4582, t17691, t4583, t1023, t17670, t4594, t17167, t977, t17171, t17157, t2979);
        let t18007 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1389(t1023, t5677, t10408, t1036, t5905, t1041, t10876, t10883, t10952, t13995, t14158, t14160, t17972, t17976, t17980, t17984, t17988, t17991, t17994, t3070, t3109, t4579, t5869, t5880, t973);
        let (t18008, t18010, t18016, t18021, t18024) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1390(t4571, t4644, t1031, t5904, t1022, t1539, t14211, t3071, t1023, t5685, t1616, t4343);
        let (t18028, t18044) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1391(t18024, t3071, t1009, t5848, t1011, t1019, t5873, t884, t10422, t5908, t3070, t1025, t10403, t10923, t10937, t14194, t14203, t14495, t14503, t18008, t18010, t18016, t18021, t3117, t378, t5900, t5909);
        let (t18047, t18050) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1392(t17614, t17640, t17684, t17725, t17900, t17967, t18007, t18044, t349, t1052, t1066, t17575, t17579, t17583, t17588, t3026, t3169, t388, t4557, t4660, t4665, t4694, t5920, t5944);
        let (t18053, t18057, t18059, t18062, t18065, t18070) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393(t5914, t990, t17875, t381, t1049, t5848, t1065, t5943, t3174, t1625, t4552, t5919);
    (t18028, t18047, t18050, t18053, t18057, t18059, t18062, t18065, t18070)
}
