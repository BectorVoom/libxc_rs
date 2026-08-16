//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2705/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2705(t12156: f64, t12303: f64, t12477: f64, t1390: f64, t16153: f64, t16490: f64, t1845: f64, t193: f64, t3918: f64, t3919: f64, t39483: f64, t5122: f64, t5126: f64, t5187: f64, t54404: f64, t54406: f64, t54409: f64, t54411: f64, t54413: f64) -> f64 {
    let t55217 = 6.0_f64 * t12156 * t1390 * t1845 * t193 + 18.0_f64 * t12303 * t5122 * t5126 - 9.0_f64 * t12477 * t3918 * t5187 + 18.0_f64 * t16153 * t3919 * t5126 + 18.0_f64 * t16490 * t193 * t5187 + t39483 - t54404 - t54406 + t54409 + t54411 - t54413;
    t55217
}
