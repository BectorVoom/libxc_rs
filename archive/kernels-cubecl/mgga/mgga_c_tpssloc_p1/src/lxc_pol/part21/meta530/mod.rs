//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2187;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta530<F: Float>(t18024: F, t3071: F, t1009: F, t5848: F, t1011: F, t1019: F, t5873: F, t884: F, t10422: F, t5908: F, t3070: F, t1025: F, t10403: F, t10923: F, t10937: F, t14194: F, t14203: F, t14495: F, t14503: F, t18008: F, t18010: F, t18016: F, t18021: F, t3117: F, t378: F, t5900: F, t5909: F, t17614: F, t17640: F, t17684: F, t17725: F, t17900: F, t17967: F, t18007: F, t349: F, t1052: F, t1066: F, t17575: F, t17579: F, t17583: F, t17588: F, t3026: F, t3169: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t5920: F, t5944: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t18025, t18028, t18029, t18030, t18035, t18036, t18041, t18044) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2187::<F>(t18024, t3071, t1009, t5848, t1011, t1019, t5873, t884, t10422, t5908, t3070, t1025, t10403, t10923, t10937, t14194, t14203, t14495, t14503, t18008, t18010, t18016, t18021, t3117, t378, t5900, t5909);
        let (t18047, t18048, t18050) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2188::<F>(t17614, t17640, t17684, t17725, t17900, t17967, t18007, t18044, t349, t1052, t1066, t17575, t17579, t17583, t17588, t3026, t3169, t388, t4557, t4660, t4665, t4694, t5920, t5944);
    (t18025, t18028, t18029, t18030, t18035, t18036, t18041, t18047, t18048, t18050)
}
