//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2208;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta707<F: Float>(t1408: F, t4303: F, t5664: F, t868: F, t86716: F, t776: F, t25373: F, t1530: F, t4119: F, t22960: F, t5660: F, t67164: F, t193: F, t7637: F, t67123: F, t1877: F, t1915: F, t22959: F, t23290: F, t25028: F, t2522: F, t25358: F, t25372: F, t25375: F, t25381: F, t28448: F, t28462: F, t6542: F, t6670: F, t7541: F, t7545: F, t86836: F) -> (F, F, F, F, F, F, F) {
        let (t97990, t97999, t98000, t98003, t98004, t98007, t98008, t98011, t98012, t98015) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2208::<F>(t1408, t4303, t5664, t868, t86716, t776, t25373, t1530, t4119, t22960, t5660, t67164);
        let (t98027, t98030, t98039) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2209::<F>(t1408, t4119, t193, t7637, t1530, t4303, t25373, t22960, t67123, t1877, t1915, t22959, t23290, t25028, t2522, t25358, t25372, t25375, t25381, t28448, t28462, t6542, t6670, t7541, t7545, t86836, t97990, t98000, t98004, t98008, t98012, t98015);
    (t97999, t98003, t98007, t98011, t98027, t98030, t98039)
}
