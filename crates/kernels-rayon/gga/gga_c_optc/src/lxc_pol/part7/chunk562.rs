//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 562/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk562(t2785: f64, t914: f64, t2586: f64, t942: f64, t940: f64, t284: f64, t853: f64, t928: f64) -> (f64, f64, f64, f64) {
    let t2786 = t914 * t2785;
    let t2789 = t2586 * t942;
    let t2790 = t940 * t2789;
    let t2797 = t928 * t853 * t284;
    (t2786, t2789, t2790, t2797)
}
