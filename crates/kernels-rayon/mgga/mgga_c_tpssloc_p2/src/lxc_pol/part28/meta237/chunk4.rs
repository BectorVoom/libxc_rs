//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1037/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1037(t5165: f64, t5360: f64, t113: f64, t1266: f64, t1271: f64, t1393: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t2314: f64, t4026: f64, t4028: f64, t4034: f64, t4037: f64, t4073: f64, t4077: f64, t510: f64, t5107: f64, t5118: f64, t513: f64, t574: f64, t650: f64, t652: f64, t672: f64) -> (f64, f64) {
    let t5361 = t5165 + t5360;
    let t5363 = -t113 * t5107 - t1266 * t1442 + t1271 * t1849 + t1393 * t1778 - 2.0_f64 * t1459 * t2314 - 2.0_f64 * t1459 * t4034 - t1774 * t650 - t4026 * t510 - 2.0_f64 * t4028 * t672 - 2.0_f64 * t4037 * t652 - 2.0_f64 * t4073 * t652 - 2.0_f64 * t4077 * t652 + t5118 * t574 + t513 * t5361;
    (t5361, t5363)
}
