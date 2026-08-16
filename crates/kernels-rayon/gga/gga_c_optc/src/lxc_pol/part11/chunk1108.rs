//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1108/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1108(t2992: f64, t5165: f64, t3058: f64, t5197: f64, t2916: f64, t2973: f64, t5117: f64, t2934: f64, t5218: f64, t8700: f64, t5311: f64, t8487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44583 = t5165 * t2992;
    let t44742 = t5197 * t3058;
    let t44909 = t5197 * t2916;
    let t44914 = t5117 * t2973;
    let t45045 = t5117 * t2934;
    let t45062 = t5218 * t8700;
    let t45304 = t8487 * t5311;
    (t44583, t44742, t44909, t44914, t45045, t45062, t45304)
}
