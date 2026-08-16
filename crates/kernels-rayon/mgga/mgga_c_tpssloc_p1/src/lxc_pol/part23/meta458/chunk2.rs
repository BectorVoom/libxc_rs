//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1338/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1338(t41741: f64, t47787: f64, t59657: f64, t68442: f64, t76574: f64, t76578: f64, t76583: f64, t76587: f64, t76591: f64, t76595: f64, t76599: f64, t20217: f64, t4337: f64) -> (f64, f64) {
    let t76602 = 0.38456790123456790123e-1_f64 * t47787 - 0.27469135802469135803e-1_f64 * t76574 - 0.92708333333333333333e-2_f64 * t76578 - 0.16481481481481481482e-1_f64 * t59657 + 0.12361111111111111111e0_f64 * t76583 - 0.61805555555555555555e-1_f64 * t76587 - 0.22249999999999999999e0_f64 * t76591 + 0.22249999999999999999e0_f64 * t76595 - 0.18541666666666666666e-1_f64 * t76599 + t41741 + 0.74166666666666666668e-1_f64 * t68442;
    let t76608 = t4337 * t20217;
    (t76602, t76608)
}
