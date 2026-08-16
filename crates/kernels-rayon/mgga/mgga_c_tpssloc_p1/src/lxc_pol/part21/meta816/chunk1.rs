//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2874/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874(t48140: f64, t48143: f64, t55716: f64, t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64) -> (f64, f64) {
    let t60091 = t48140 * t48143 * t55716;
    let t60106 = 32.0_f64 / 27.0_f64 * t47705 - 32.0_f64 / 81.0_f64 * t47707 + 8.0_f64 / 27.0_f64 * t47709 + 4.0_f64 / 27.0_f64 * t47711 + 20.0_f64 / 81.0_f64 * t47713 - 8.0_f64 / 9.0_f64 * t47715 - 4.0_f64 / 9.0_f64 * t47717 - 8.0_f64 / 9.0_f64 * t47724 - 16.0_f64 / 27.0_f64 * t47730 + 2.0_f64 / 9.0_f64 * t47732 + 4.0_f64 / 3.0_f64 * t47738 - 4.0_f64 / 27.0_f64 * t41656;
    (t60091, t60106)
}
