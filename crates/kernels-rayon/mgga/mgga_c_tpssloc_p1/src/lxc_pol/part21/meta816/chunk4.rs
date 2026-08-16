//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2877/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877(t47787: f64, t59727: f64, t59732: f64, t59735: f64, t59738: f64, t59744: f64, t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t59769: f64) -> f64 {
    let t60147 = -10.0_f64 / 27.0_f64 * t59727 + 4.0_f64 / 3.0_f64 * t59732 - 40.0_f64 / 27.0_f64 * t59735 + 16.0_f64 / 3.0_f64 * t59738 + 56.0_f64 / 81.0_f64 * t47787 + 4.0_f64 / 3.0_f64 * t59744 - 2.0_f64 * t59748 + 40.0_f64 / 9.0_f64 * t59753 - 8.0_f64 * t59757 + 4.0_f64 / 3.0_f64 * t59759 - 8.0_f64 / 9.0_f64 * t59761 - 2.0_f64 * t59765 + 4.0_f64 / 3.0_f64 * t59769;
    t60147
}
