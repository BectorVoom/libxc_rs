//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2876/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876(t41904: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64) -> f64 {
    let t60133 = 8.0_f64 / 27.0_f64 * t59688 + 4.0_f64 / 3.0_f64 * t59692 - 4.0_f64 / 27.0_f64 * t59694 + t41904 + 2.0_f64 / 3.0_f64 * t59698 - 8.0_f64 / 9.0_f64 * t59700 + 8.0_f64 / 27.0_f64 * t59702 + 20.0_f64 / 81.0_f64 * t59704 - 10.0_f64 / 27.0_f64 * t59708 - 80.0_f64 / 81.0_f64 * t59713 + 4.0_f64 / 3.0_f64 * t59717 - 4.0_f64 / 9.0_f64 * t59721;
    t60133
}
