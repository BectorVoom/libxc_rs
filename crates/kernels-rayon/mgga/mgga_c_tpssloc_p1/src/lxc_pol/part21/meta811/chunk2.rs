//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2845/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2845(t41741: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64) -> f64 {
    let t59723 = 0.82407407407407407409e-2_f64 * t59688 + 0.37083333333333333334e-1_f64 * t59692 - 0.41203703703703703704e-2_f64 * t59694 + t41741 + 0.18541666666666666667e-1_f64 * t59698 - 0.24722222222222222222e-1_f64 * t59700 + 0.82407407407407407407e-2_f64 * t59702 + 0.68672839506172839506e-2_f64 * t59704 - 0.10300925925925925926e-1_f64 * t59708 - 0.27469135802469135803e-1_f64 * t59713 + 0.37083333333333333333e-1_f64 * t59717 - 0.12361111111111111111e-1_f64 * t59721;
    t59723
}
