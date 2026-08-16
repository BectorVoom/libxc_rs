//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta389 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1388;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1389;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1390;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1391;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1392;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta389<F: Float>(t14164: F, t17686: F, t4582: F, t17691: F, t4583: F, t1023: F, t17670: F, t4594: F, t17167: F, t977: F, t17171: F, t17157: F, t2979: F, t5677: F, t10408: F, t1036: F, t5905: F, t1041: F, t10876: F, t10883: F, t10952: F, t13995: F, t14158: F, t14160: F, t3070: F, t3109: F, t4579: F, t5869: F, t5880: F, t973: F, t4571: F, t4644: F, t1031: F, t5904: F, t1022: F, t1539: F, t14211: F, t3071: F, t5685: F, t1616: F, t4343: F, t1009: F, t5848: F, t1011: F, t1019: F, t5873: F, t884: F, t10422: F, t5908: F, t1025: F, t10403: F, t10923: F, t10937: F, t14194: F, t14203: F, t14495: F, t14503: F, t3117: F, t378: F, t5900: F, t5909: F, t17614: F, t17640: F, t17684: F, t17725: F, t17900: F, t17967: F, t349: F, t1052: F, t1066: F, t17575: F, t17579: F, t17583: F, t17588: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t5920: F, t5944: F, t5914: F, t990: F, t17875: F, t381: F, t1049: F, t1065: F, t5943: F, t3174: F, t1625: F, t4552: F, t5919: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17972, t17976, t17980, t17984, t17988, t17991, t17994) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1388::<F>(t14164, t17686, t4582, t17691, t4583, t1023, t17670, t4594, t17167, t977, t17171, t17157, t2979);
        let t18007 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1389::<F>(t1023, t5677, t10408, t1036, t5905, t1041, t10876, t10883, t10952, t13995, t14158, t14160, t17972, t17976, t17980, t17984, t17988, t17991, t17994, t3070, t3109, t4579, t5869, t5880, t973);
        let (t18008, t18010, t18016, t18021, t18024) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1390::<F>(t4571, t4644, t1031, t5904, t1022, t1539, t14211, t3071, t1023, t5685, t1616, t4343);
        let (t18028, t18044) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1391::<F>(t18024, t3071, t1009, t5848, t1011, t1019, t5873, t884, t10422, t5908, t3070, t1025, t10403, t10923, t10937, t14194, t14203, t14495, t14503, t18008, t18010, t18016, t18021, t3117, t378, t5900, t5909);
        let (t18047, t18050) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1392::<F>(t17614, t17640, t17684, t17725, t17900, t17967, t18007, t18044, t349, t1052, t1066, t17575, t17579, t17583, t17588, t3026, t3169, t388, t4557, t4660, t4665, t4694, t5920, t5944);
        let (t18053, t18057, t18059, t18062, t18065, t18070) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1393::<F>(t5914, t990, t17875, t381, t1049, t5848, t1065, t5943, t3174, t1625, t4552, t5919);
    (t18028, t18047, t18050, t18053, t18057, t18059, t18062, t18065, t18070)
}
