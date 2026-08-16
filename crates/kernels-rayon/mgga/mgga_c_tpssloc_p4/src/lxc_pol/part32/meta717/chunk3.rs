//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2274/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2274(t193: f64, t200: f64, t7540: f64, t1408: f64, t16557: f64, t1877: f64, t1915: f64, t22959: f64, t23295: f64, t25: f64, t25013: f64, t25015: f64, t25021: f64, t2522: f64, t25354: f64, t25366: f64, t25372: f64, t25385: f64, t7541: f64, t86736: f64, t98091: f64, t98094: f64, t98103: f64, t98112: f64, t99043: f64, t99049: f64, t99055: f64, t99056: f64, t99060: f64) -> (f64, f64) {
    let t99064 = t193 * t200 * t7540;
    let t99067 = t1877 * t23295 * t98091 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t98094 - 3.0_f64 * t86736 * t25021 - 3.0_f64 * t86736 * t25366 + t25372 * t98103 + 3.0_f64 * t2522 * t7541 * t25385 + t1877 * t1915 * t16557 / 2.0_f64 + 6.0_f64 * t22959 * t98112 + t1877 * t99043 * t25 / 2.0_f64 + t1877 * t25354 * t1408 - 3.0_f64 * t22959 * t99049 + t99055 + 3.0_f64 * t25013 * t99056 + 6.0_f64 * t25013 * t99060 + 6.0_f64 * t99064 * t25015;
    (t99064, t99067)
}
