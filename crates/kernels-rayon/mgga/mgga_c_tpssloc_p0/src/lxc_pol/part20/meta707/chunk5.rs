//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2703/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2703(t16486: f64, t3701: f64, t1388: f64, t3914: f64, t15899: f64, t16148: f64, t16497: f64, t3719: f64, t3918: f64, t3919: f64, t39338: f64, t39346: f64, t39349: f64, t39356: f64, t5126: f64, t5160: f64, t54321: f64, t54322: f64, t54324: f64) -> f64 {
    let t55169 = t16486 * t3701;
    let t55173 = t3914 * t1388;
    let t55180 = -3.0_f64 * t1388 * t5160 * t55169 + 6.0_f64 * t15899 * t5160 * t55173 + 36.0_f64 * t16148 * t3919 * t5126 + 9.0_f64 * t16497 * t3719 * t3918 - t39338 + t39346 + t39349 + t39356 + t54321 - t54322 + t54324;
    t55180
}
