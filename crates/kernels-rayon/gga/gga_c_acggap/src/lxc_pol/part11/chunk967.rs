//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 967/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk967(t32029: f64, t464: f64, t2122: f64, t323: f64, t851: f64, t14575: f64, t7932: f64, t7942: f64, t16020: f64, t7884: f64, t7941: f64, t15758: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32030 = t32029 * t464;
    let t32033 = t851 * t2122 * t323;
    let t32036 = t7942 * t7932 * t14575;
    let t32039 = t7942 * t7932 * t16020;
    let t32041 = t7884 * t7941;
    let t32043 = t32041 * t7932 * t15758;
    (t32030, t32033, t32036, t32039, t32041, t32043)
}
