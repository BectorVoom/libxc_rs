//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2402/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2402(t41741: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> f64 {
    let t68756 = 0.55625000000000000001e-1_f64 * t68586 + 0.18541666666666666667e-1_f64 * t68589 - 0.61805555555555555555e-2_f64 * t68592 + 0.12361111111111111111e0_f64 * t68596 - 0.30902777777777777778e-1_f64 * t68599 + 0.11125e0_f64 * t68602 - 0.30902777777777777777e-1_f64 * t68605 - 0.166875e0_f64 * t68608 - 0.18541666666666666667e-1_f64 * t59663 + 0.61805555555555555556e-2_f64 * t59665 + 0.92708333333333333334e-2_f64 * t59680 + 0.24722222222222222223e-1_f64 * t59688 - 0.12361111111111111111e-1_f64 * t59694 + t41741 - 0.37083333333333333333e-1_f64 * t59700 + 0.12361111111111111111e-1_f64 * t59702 + 0.10300925925925925926e-1_f64 * t59704 + 0.28842592592592592592e-1_f64 * t47787 + 0.55625000000000000001e-1_f64 * t59759 - 0.37083333333333333334e-1_f64 * t59761;
    t68756
}
