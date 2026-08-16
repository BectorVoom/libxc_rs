//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2398/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2398(t41904: f64, t47787: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t59700: f64, t59702: f64, t59704: f64, t59759: f64, t59761: f64, t68586: f64, t68589: f64, t68592: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> f64 {
    let t68693 = 2.0_f64 * t68586 + 2.0_f64 / 3.0_f64 * t68589 - 2.0_f64 / 9.0_f64 * t68592 + 40.0_f64 / 9.0_f64 * t68596 - 10.0_f64 / 9.0_f64 * t68599 + 4.0_f64 * t68602 - 10.0_f64 / 9.0_f64 * t68605 - 6.0_f64 * t68608 - 2.0_f64 / 3.0_f64 * t59663 + 2.0_f64 / 9.0_f64 * t59665 + t59680 / 3.0_f64 + 8.0_f64 / 9.0_f64 * t59688 - 4.0_f64 / 9.0_f64 * t59694 + t41904 - 4.0_f64 / 3.0_f64 * t59700 + 4.0_f64 / 9.0_f64 * t59702 + 10.0_f64 / 27.0_f64 * t59704 + 28.0_f64 / 27.0_f64 * t47787 + 2.0_f64 * t59759 - 4.0_f64 / 3.0_f64 * t59761;
    t68693
}
