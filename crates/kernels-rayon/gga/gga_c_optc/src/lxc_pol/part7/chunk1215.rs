//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1215/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1215(t309: f64, t7898: f64, t300: f64, t2587: f64, t7886: f64, t2586: f64, t7866: f64, t893: f64, t7298: f64, t896: f64, t22015: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25071 = t7898 * t309;
    let t25072 = t300 * t25071;
    let t25075 = t7886 * t2587;
    let t25077 = t2586 * t7866;
    let t25078 = t893 * t25077;
    let t25085 = t896 * t7298;
    let t25087 = t894 * t25085 * t22015;
    (t25071, t25072, t25075, t25077, t25078, t25087)
}
