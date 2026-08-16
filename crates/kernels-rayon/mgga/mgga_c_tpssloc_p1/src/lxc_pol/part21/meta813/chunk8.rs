//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2865/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2865(t41655: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64) -> f64 {
    let t59873 = 0.15829629629629629629e-1_f64 * t59688 + 0.71233333333333333332e-1_f64 * t59692 - 0.79148148148148148146e-2_f64 * t59694 + t41655 + 0.35616666666666666666e-1_f64 * t59698 - 0.47488888888888888888e-1_f64 * t59700 + 0.15829629629629629629e-1_f64 * t59702 + 0.13191358024691358024e-1_f64 * t59704 - 0.19787037037037037037e-1_f64 * t59708 - 0.52765432098765432099e-1_f64 * t59713 + 0.71233333333333333332e-1_f64 * t59717 - 0.23744444444444444444e-1_f64 * t59721;
    t59873
}
