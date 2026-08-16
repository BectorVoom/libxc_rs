//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 940/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk940(t1595: f64, t8690: f64, t1655: f64, t38042: f64, t38044: f64, t38048: f64, t38055: f64, t38059: f64, t38066: f64, t38073: f64, t38092: f64, t38094: f64, t38096: f64, t38098: f64, t38105: f64, t7977: f64, t8693: f64) -> (f64, f64) {
    let t39563 = t8690 * t1595;
    let t39568 = -0.38514888888888888888e0_f64 * t38092 - 0.25676592592592592592e0_f64 * t38094 + 0.21397160493827160493e0_f64 * t38096 + 0.19257444444444444444e0_f64 * t38098 + 0.86658499999999999998e0_f64 * t38105 + 0.77029777777777777776e0_f64 * t38042 - 0.77029777777777777776e0_f64 * t38044 + 0.11554466666666666666e1_f64 * t38048 - 0.42794320987654320987e0_f64 * t38055 - 0.14443083333333333333e0_f64 * t38059 + 0.19257444444444444444e1_f64 * t38066 - 0.34663399999999999999e1_f64 * t38073 + 0.1056393e1_f64 * t39563 * t1655 - 0.469508e0_f64 * t8693 * t7977;
    (t39563, t39568)
}
