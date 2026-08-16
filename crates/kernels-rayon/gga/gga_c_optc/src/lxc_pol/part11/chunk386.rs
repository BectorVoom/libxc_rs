//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 386/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk386(t97: f64, t620: f64, t34: f64, t99: f64, t115: f64, t681: f64) -> (f64, f64, f64, f64, f64) {
    let t1884 = 1.0_f64 / t97;
    let t1888 = 1.0_f64 / t620;
    let t1889 = t34 * t1888;
    let t1896 = 1.0_f64 / t99;
    let t1916 = t681 * t115;
    (t1884, t1888, t1889, t1896, t1916)
}
