//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 429/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk429(t664: f64, t669: f64, t166: f64, t668: f64, t145: f64, t708: f64) -> (f64, f64, f64, f64) {
    let t2100 = t664 * t669;
    let t2105 = 1.0_f64 / t668 / t166;
    let t2106 = t145 * t2105;
    let t2107 = t708 * t708;
    (t2100, t2105, t2106, t2107)
}
