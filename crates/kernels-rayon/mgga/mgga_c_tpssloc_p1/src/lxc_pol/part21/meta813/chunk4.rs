//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2861/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2861(t42245: f64, t59688: f64, t59692: f64, t59694: f64, t59698: f64, t59700: f64, t59702: f64, t59704: f64, t59708: f64, t59713: f64, t59717: f64, t59721: f64) -> f64 {
    let t59815 = 0.1522074074074074074e-1_f64 * t59688 + 0.68493333333333333332e-1_f64 * t59692 - 0.76103703703703703702e-2_f64 * t59694 + t42245 + 0.34246666666666666666e-1_f64 * t59698 - 0.4566222222222222222e-1_f64 * t59700 + 0.1522074074074074074e-1_f64 * t59702 + 0.12683950617283950617e-1_f64 * t59704 - 0.19025925925925925925e-1_f64 * t59708 - 0.50735802469135802467e-1_f64 * t59713 + 0.68493333333333333331e-1_f64 * t59717 - 0.2283111111111111111e-1_f64 * t59721;
    t59815
}
