//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1150/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1150(t3813: f64, t4768: f64, t16979: f64, t41818: f64, t10959: f64, t16636: f64, t3835: f64, t16644: f64, t8143: f64, t17047: f64, t874: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t50937 = t3813 * t4768;
    let t50941 = t41818 * t16979;
    let t50955 = t3835 * t10959 * t16636;
    let t50985 = t3835 * t8143 * t16644;
    let t50994 = t874 * t888 * t17047;
    (t50937, t50941, t50955, t50985, t50994)
}
