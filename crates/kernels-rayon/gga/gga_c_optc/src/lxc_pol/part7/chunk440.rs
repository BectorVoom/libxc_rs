//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 440/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk440(t106: f64, t167: f64, t2096: f64, t2100: f64, t2106: f64, t2107: f64, t2189: f64, t670: f64, t708: f64) -> f64 {
    let t2193 = 0.27818116767324025134e1_f64 * t106 * t2096 * t167 - 0.55636233534648050268e1_f64 * t106 * t2100 * t708 + 0.55636233534648050268e1_f64 * t106 * t2106 * t2107 - 0.27818116767324025134e1_f64 * t106 * t670 * t2189;
    t2193
}
