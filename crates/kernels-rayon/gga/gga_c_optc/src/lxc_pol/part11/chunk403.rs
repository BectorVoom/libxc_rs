//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 403/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk403(t136: f64, t2086: f64, t166: f64, t668: f64, t145: f64, t108: f64, t692: f64, t110: f64, t146: f64, t622: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2087 = t136 * t2086;
    let t2105 = 1.0_f64 / t668 / t166;
    let t2106 = t145 * t2105;
    let t2111 = t692 * t108;
    let t2113 = t146 * t2111 * t110;
    let t2120 = t146 * t671 * t622;
    (t2087, t2105, t2106, t2111, t2113, t2120)
}
