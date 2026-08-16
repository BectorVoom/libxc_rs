//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2221/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221(t5: f64, t12568: f64, t12585: f64, t12588: f64, t12719: f64, t1437: f64, t2235: f64, t2240: f64, t2307: f64, t39046: f64, t39063: f64, t3958: f64, t4021: f64, t45844: f64, t46114: f64, t9228: f64, t9231: f64, t9239: f64, t9240: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t46116 = piecewise3(t8, 0.0_f64, 840.0_f64 * t1437 * t39063 * t9240 + 60.0_f64 * t2240 * t2307 * t4021 - 360.0_f64 * t2307 * t3958 * t9239 - 12.0_f64 * t12568 * t2307 + 120.0_f64 * t12585 * t9231 + 60.0_f64 * t12588 * t9231 - 12.0_f64 * t12719 * t2235 - 4.0_f64 * t1437 * t39046 - 12.0_f64 * t4021 * t9228 - 120.0_f64 * t45844 * t9240 + t46114);
    t46116
}
