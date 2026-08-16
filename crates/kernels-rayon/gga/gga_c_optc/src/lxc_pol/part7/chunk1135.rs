//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1135/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1135(t23685: f64, t23651: f64, t23653: f64, t23655: f64, t23660: f64, t23664: f64, t23667: f64, t23670: f64, t23673: f64, t23676: f64, t23679: f64, t23683: f64) -> f64 {
    let t23686 = 0.20068888888888888889e-1_f64 * t23685;
    let t23687 = 0.7389e-2_f64 * t23651 - 0.15510666666666666667e2_f64 * t23653 + 0.5170222222222222222e1_f64 * t23655 + 0.15510666666666666667e2_f64 * t23660 - 0.44334e-1_f64 * t23664 + 0.9852e-2_f64 * t23667 + 0.46531999999999999998e2_f64 * t23670 - 0.5170222222222222222e1_f64 * t23673 - 0.12925555555555555555e2_f64 * t23676 - 0.69798e2_f64 * t23679 + t23683 + t23686;
    t23687
}
