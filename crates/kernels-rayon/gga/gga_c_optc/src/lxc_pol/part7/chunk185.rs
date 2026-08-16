//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 185/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk185(t106: f64, t454: f64, t470: f64, t373: f64, t383: f64) -> (f64, f64, f64) {
    let t474 = 1.0_f64 + 0.27818116767324025134e1_f64 * t106 * t454 * t470;
    let t475 = f64::ln(t474);
    let t481 = 0.2568e1_f64 + 0.58165e1_f64 * t373 + 0.184725e-2_f64 * t383;
    (t474, t475, t481)
}
