//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2187;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta530(t18024: f64, t3071: f64, t1009: f64, t5848: f64, t1011: f64, t1019: f64, t5873: f64, t884: f64, t10422: f64, t5908: f64, t3070: f64, t1025: f64, t10403: f64, t10923: f64, t10937: f64, t14194: f64, t14203: f64, t14495: f64, t14503: f64, t18008: f64, t18010: f64, t18016: f64, t18021: f64, t3117: f64, t378: f64, t5900: f64, t5909: f64, t17614: f64, t17640: f64, t17684: f64, t17725: f64, t17900: f64, t17967: f64, t18007: f64, t349: f64, t1052: f64, t1066: f64, t17575: f64, t17579: f64, t17583: f64, t17588: f64, t3026: f64, t3169: f64, t388: f64, t4557: f64, t4660: f64, t4665: f64, t4694: f64, t5920: f64, t5944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18025, t18028, t18029, t18030, t18035, t18036, t18041, t18044) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2187(t18024, t3071, t1009, t5848, t1011, t1019, t5873, t884, t10422, t5908, t3070, t1025, t10403, t10923, t10937, t14194, t14203, t14495, t14503, t18008, t18010, t18016, t18021, t3117, t378, t5900, t5909);
        let (t18047, t18048, t18050) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2188(t17614, t17640, t17684, t17725, t17900, t17967, t18007, t18044, t349, t1052, t1066, t17575, t17579, t17583, t17588, t3026, t3169, t388, t4557, t4660, t4665, t4694, t5920, t5944);
    (t18025, t18028, t18029, t18030, t18035, t18036, t18041, t18047, t18048, t18050)
}
