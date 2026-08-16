//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1238/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1238(t2587: f64, t8220: f64, t2367: f64, t7406: f64, t930: f64, t322: f64, t8192: f64) -> (f64, f64, f64) {
    let t25547 = t8220 * t2587;
    let t25552 = t930 * t2367 * t7406;
    let t25560 = t8192 * t322;
    (t25547, t25552, t25560)
}
