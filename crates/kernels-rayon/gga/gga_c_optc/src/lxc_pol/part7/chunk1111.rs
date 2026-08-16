//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1111/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1111(t23581: f64, t23583: f64, t23585: f64, t23587: f64, t23592: f64, t23597: f64, t23602: f64, t23605: f64, t23608: f64, t23612: f64, t23614: f64, t23616: f64) -> f64 {
    let t23618 = -0.12315e-2_f64 * t23581 - 0.821e-2_f64 * t23583 + 0.3284e-2_f64 * t23585 - 0.19704e-1_f64 * t23587 + 0.14778e-1_f64 * t23592 - 0.1642e-2_f64 * t23597 - 0.3284e-2_f64 * t23602 - 0.46531999999999999999e2_f64 * t23605 - 0.38776666666666666665e1_f64 * t23608 + 0.46532e2_f64 * t23612 + 0.10340444444444444444e2_f64 * t23614 + 0.15510666666666666667e2_f64 * t23616;
    t23618
}
