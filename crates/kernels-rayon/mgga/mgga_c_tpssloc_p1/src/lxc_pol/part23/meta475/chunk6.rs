//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1426/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1426(t43942: f64, t50834: f64, t71154: f64, t71156: f64, t77998: f64, t78002: f64, t78005: f64, t78033: f64, t78037: f64, t78041: f64, t78045: f64, t78049: f64) -> f64 {
    let t78223 = -0.12361111111111111111e-1_f64 * t71154 + 0.55625000000000000001e-1_f64 * t77998 + 0.49444444444444444444e-1_f64 * t71156 + 0.12361111111111111111e0_f64 * t78002 - 0.24722222222222222222e-1_f64 * t78033 - 0.38456790123456790123e-1_f64 * t50834 + t43942 + 0.61805555555555555555e-1_f64 * t78037 - 0.22249999999999999999e0_f64 * t78041 + 0.33375e0_f64 * t78045 + 0.74166666666666666668e-1_f64 * t78049 - 0.18541666666666666666e-1_f64 * t78005;
    t78223
}
