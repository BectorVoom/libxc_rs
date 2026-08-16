//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 933/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk933(t5: f64, t12571: f64, t1437: f64, t19299: f64, t20193: f64, t20201: f64, t20204: f64, t20288: f64, t2240: f64, t3953: f64, t5389: f64, t5445: f64, t605: f64, t86: f64, t9239: f64) -> f64 {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t20292 = piecewise3(t8, 0.0_f64, 60.0_f64 * t12571 * t5389 - 12.0_f64 * t1437 * t19299 + t20193 * t86 - 120.0_f64 * t20201 * t9239 + 60.0_f64 * t20204 * t2240 - 4.0_f64 * t20288 * t605 - 12.0_f64 * t3953 * t5445);
    t20292
}
