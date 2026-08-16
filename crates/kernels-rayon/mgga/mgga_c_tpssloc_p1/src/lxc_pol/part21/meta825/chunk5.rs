//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2905/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2905(t41831: f64, t47705: f64, t47707: f64, t47709: f64, t47711: f64, t47713: f64, t47715: f64, t47717: f64, t47724: f64, t47730: f64, t47732: f64, t48087: f64, t48096: f64, t48098: f64) -> f64 {
    let t60585 = 0.83356000000000000002e0_f64 * t48087 + 0.18363555555555555555e1_f64 * t47705 - 0.6121185185185185185e0_f64 * t47707 + 0.45908888888888888888e0_f64 * t47709 + 0.22954444444444444444e0_f64 * t47711 + 0.38257407407407407407e0_f64 * t47713 - 0.13772666666666666666e1_f64 * t47715 - 0.68863333333333333332e0_f64 * t47717 - 0.13772666666666666666e1_f64 * t47724 + 0.23154444444444444444e0_f64 * t41831 - 0.4630888888888888889e0_f64 * t48096 + 0.13892666666666666667e0_f64 * t48098 - 0.91817777777777777776e0_f64 * t47730 + 0.34431666666666666666e0_f64 * t47732;
    t60585
}
