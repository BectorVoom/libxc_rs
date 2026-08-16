//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1430/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1430(t44348: f64, t50834: f64, t71154: f64, t71156: f64, t77998: f64, t78002: f64, t78005: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64) -> f64 {
    let t78278 = -0.23744444444444444444e-1_f64 * t71154 + 0.10685e0_f64 * t77998 + 0.94977777777777777776e-1_f64 * t71156 + 0.23744444444444444444e0_f64 * t78002 - 0.47488888888888888888e-1_f64 * t78033 - 0.73871604938271604937e-1_f64 * t50834 + t44348 + 0.11872222222222222222e0_f64 * t78037 - 0.42739999999999999999e0_f64 * t78041 + 0.6411e0_f64 * t78045 + 0.14246666666666666667e0_f64 * t78049 - 0.35616666666666666666e-1_f64 * t78005;
    t78278
}
