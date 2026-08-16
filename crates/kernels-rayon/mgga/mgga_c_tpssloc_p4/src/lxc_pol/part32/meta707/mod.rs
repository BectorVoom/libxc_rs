//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta707 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2208;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2209;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta707(t1408: f64, t4303: f64, t5664: f64, t868: f64, t86716: f64, t776: f64, t25373: f64, t1530: f64, t4119: f64, t22960: f64, t5660: f64, t67164: f64, t193: f64, t7637: f64, t67123: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25028: f64, t2522: f64, t25358: f64, t25372: f64, t25375: f64, t25381: f64, t28448: f64, t28462: f64, t6542: f64, t6670: f64, t7541: f64, t7545: f64, t86836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97990, t97999, t98000, t98003, t98004, t98007, t98008, t98011, t98012, t98015) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2208(t1408, t4303, t5664, t868, t86716, t776, t25373, t1530, t4119, t22960, t5660, t67164);
        let (t98027, t98030, t98039) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2209(t1408, t4119, t193, t7637, t1530, t4303, t25373, t22960, t67123, t1877, t1915, t22959, t23290, t25028, t2522, t25358, t25372, t25375, t25381, t28448, t28462, t6542, t6670, t7541, t7545, t86836, t97990, t98000, t98004, t98008, t98012, t98015);
    (t97999, t98003, t98007, t98011, t98027, t98030, t98039)
}
