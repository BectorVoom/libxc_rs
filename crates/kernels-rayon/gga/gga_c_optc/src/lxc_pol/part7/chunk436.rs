//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 436/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk436(t131: f64, t2156: f64, t133: f64, t155: f64, t2025: f64, t696: f64, t652: f64, t693: f64) -> (f64, f64, f64, f64) {
    let t2157 = t2156 * t131;
    let t2159 = t155 * t2157 * t133;
    let t2160 = t696 * t2025;
    let t2164 = t155 * t693 * t652;
    (t2157, t2159, t2160, t2164)
}
