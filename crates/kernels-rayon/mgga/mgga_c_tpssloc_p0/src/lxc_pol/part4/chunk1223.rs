//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1223/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1223(t19595: f64, t20075: f64, t20092: f64, t20096: f64, t19534: f64, t510: f64, t1458: f64, t5107: f64, t113: f64, t12725: f64, t1442: f64, t1459: f64, t1774: f64, t1778: f64, t1849: f64, t19289: f64, t19537: f64, t2314: f64, t4026: f64, t4028: f64, t4034: f64, t4073: f64, t4077: f64, t5118: f64, t513: f64, t5361: f64, t5460: f64, t574: f64, t652: f64, t7458: f64) -> f64 {
    let t20098 = t19595 + t20075 + t20092 + t20096;
    let t20100 = t510 * t19534;
    let t20109 = t5107 * t1458;
    let t20118 = -t113 * t19289 - 4.0_f64 * t12725 * t1459 - 2.0_f64 * t1442 * t5107 - 2.0_f64 * t1774 * t4026 + 2.0_f64 * t1778 * t5361 + 2.0_f64 * t1849 * t5118 + t19537 * t574 + t20098 * t513 - 2.0_f64 * t20100 * t652 - 4.0_f64 * t20109 * t652 - 4.0_f64 * t2314 * t5460 - 4.0_f64 * t4028 * t4073 - 4.0_f64 * t4028 * t4077 - 4.0_f64 * t4034 * t5460 - 4.0_f64 * t4073 * t7458;
    t20118
}
