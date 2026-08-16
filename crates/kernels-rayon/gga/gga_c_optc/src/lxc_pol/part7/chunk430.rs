//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 430/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk430(t108: f64, t692: f64, t110: f64, t146: f64, t2022: f64, t5: f64, t2024: f64, t675: f64, t622: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t2111 = t692 * t108;
    let t2113 = t146 * t2111 * t110;
    let t2114 = t5 * t2022;
    let t2115 = t2114 * t2024;
    let t2116 = t675 * t2115;
    let t2120 = t146 * t671 * t622;
    (t2111, t2113, t2114, t2116, t2120)
}
