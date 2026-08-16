//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 860/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk860(t1: f64, t8209: f64, t8195: f64, t3916: f64, t8193: f64, t2672: f64, t935: f64, t7885: f64, t952: f64, t2587: f64, t2704: f64, t2743: f64, t921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8210 = t8209 * t1;
    let t8211 = t8195 * t8210;
    let t8214 = t3916 * t8193;
    let t8215 = t2672 * t935;
    let t8216 = t8215 * t1;
    let t8217 = t8195 * t8216;
    let t8220 = t952 * t7885;
    let t8223 = t2704 * t2587;
    let t8226 = t921 * t2743;
    (t8210, t8211, t8214, t8215, t8216, t8217, t8220, t8223, t8226)
}
