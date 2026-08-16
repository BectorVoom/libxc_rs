//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1076/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1076(t1113: f64, t191: f64, t529: f64, t1561: f64, t9303: f64, t2229: f64, t4759: f64, t108: f64, t176: f64, t203: f64, t2226: f64, t4595: f64) -> (f64, f64, f64, f64) {
    let t36863 = t529 * t1113 * t191;
    let t36985 = t1561 * t9303;
    let t37138 = t2229 * t4759;
    let t37152 = t176 * t2226 * t4595 * t108 * t203;
    (t36863, t36985, t37138, t37152)
}
