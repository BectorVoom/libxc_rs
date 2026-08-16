//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2440/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2440(t42245: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> f64 {
    let t69445 = 0.10274e0_f64 * t68586 + 0.34246666666666666666e-1_f64 * t68589 - 0.11415555555555555555e-1_f64 * t68592 + 0.2283111111111111111e0_f64 * t68596 - 0.57077777777777777775e-1_f64 * t68599 + 0.20547999999999999999e0_f64 * t68602 - 0.57077777777777777775e-1_f64 * t68605 - 0.30822e0_f64 * t68608 - 0.34246666666666666666e-1_f64 * t59663 + 0.11415555555555555555e-1_f64 * t59665 + 0.17123333333333333333e-1_f64 * t59680 + 0.4566222222222222222e-1_f64 * t59688 - 0.22831111111111111111e-1_f64 * t59694 + t42245 - 0.6849333333333333333e-1_f64 * t59700 + 0.2283111111111111111e-1_f64 * t59702 + 0.19025925925925925925e-1_f64 * t59704 + 0.5327259259259259259e-1_f64 * t47787 + 0.10274e0_f64 * t59759 - 0.6849333333333333333e-1_f64 * t59761;
    t69445
}
