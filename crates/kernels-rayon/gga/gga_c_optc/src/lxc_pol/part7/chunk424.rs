//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 424/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk424(t127: f64, t2067: f64, t6: f64, t161: f64, t2023: f64, t2040: f64, t22: f64) -> (f64, f64, f64, f64, f64) {
    let t2069 = t6 * t2067 * t127;
    let t2070 = t161 * t2069;
    let t2073 = t2023 * t127;
    let t2074 = t161 * t2073;
    let t2078 = 1.0_f64 / t22 / t2040;
    (t2069, t2070, t2073, t2074, t2078)
}
