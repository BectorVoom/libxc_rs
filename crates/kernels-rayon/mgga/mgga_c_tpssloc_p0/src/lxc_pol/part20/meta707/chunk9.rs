//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2707/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2707(t12461: f64, t5356: f64, t1388: f64, t3719: f64, t19577: f64, t22578: f64, t3698: f64, t3918: f64, t39367: f64, t39585: f64, t39590: f64, t39593: f64, t39595: f64, t5160: f64, t5161: f64, t54433: f64, t54435: f64, t54436: f64) -> f64 {
    let t55242 = t5356 * t12461;
    let t55246 = t1388 * t3719;
    let t55256 = -9.0_f64 * t19577 * t22578 * t3918 + 6.0_f64 * t3698 * t5160 * t55242 - 9.0_f64 * t3918 * t39367 * t5161 - 9.0_f64 * t3918 * t5161 * t55246 - t39585 + t39590 - t39593 + t39595 + t54433 - t54435 + t54436;
    t55256
}
