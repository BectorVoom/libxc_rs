//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 922/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk922(t141: f64, t7335: f64, t301: f64, t30407: f64, t7325: f64, t1016: f64, t1072: f64, t30418: f64, t372: f64, t3201: f64, t7486: f64, t2095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31097 = t7335 * t141;
    let t31100 = t30407 * t31097 * t7325 * t301;
    let t31102 = t1016 * t1072;
    let t31105 = t30407 * t30418 * t31102 * t372;
    let t31107 = t3201 * t7486;
    let t31108 = t2095 * t31107;
    (t31097, t31100, t31102, t31105, t31107, t31108)
}
