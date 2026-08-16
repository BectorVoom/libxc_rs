//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2708/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2708(t193: f64, t3734: f64, t1845: f64, t40611: f64, t12458: f64, t1307: f64, t15868: f64, t15883: f64, t15904: f64, t16018: f64, t3719: f64, t3918: f64, t39639: f64, t5126: f64, t5131: f64, t5160: f64, t54447: f64, t54448: f64, t54449: f64, t54450: f64, t54452: f64, t571: f64) -> f64 {
    let t55266 = t193 * t3734;
    let t55276 = t1845 * t40611;
    let t55280 = 18.0_f64 * t1307 * t16018 * t5126 * t571 - 6.0_f64 * t12458 * t5160 * t55276 - 18.0_f64 * t15868 * t15904 * t3918 + 18.0_f64 * t15883 * t3719 * t5126 + 18.0_f64 * t5131 * t55266 + t39639 - t54447 - t54448 + t54449 - t54450 + t54452;
    t55280
}
