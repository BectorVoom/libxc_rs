//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 563/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk563(t2367: f64, t866: f64, t930: f64, t2629: f64, t914: f64, t2634: f64, t2587: f64, t953: f64, t301: f64, t938: f64, t873: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2800 = t2367 * t866;
    let t2801 = t930 * t2800;
    let t2803 = t914 * t2629;
    let t2806 = t914 * t2634;
    let t2809 = t953 * t2587;
    let t2811 = t938 * t301;
    let t2812 = t2811 * t873;
    (t2800, t2801, t2803, t2806, t2809, t2811, t2812)
}
