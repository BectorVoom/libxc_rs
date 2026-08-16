//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2863/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2863(t41656: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t47738: f64) -> f64 {
    let t59846 = 0.63318518518518518517e-1_f64 * t47705 - 0.21106172839506172839e-1_f64 * t47707 + 0.15829629629629629629e-1_f64 * t47709 + 0.79148148148148148147e-2_f64 * t47711 + 0.13191358024691358025e-1_f64 * t47713 - 0.47488888888888888888e-1_f64 * t47715 - 0.23744444444444444444e-1_f64 * t47717 - 0.47488888888888888888e-1_f64 * t47724 - 0.31659259259259259258e-1_f64 * t47730 + 0.11872222222222222222e-1_f64 * t47732 + 0.71233333333333333332e-1_f64 * t47738 - 0.79148148148148148147e-2_f64 * t41656;
    t59846
}
