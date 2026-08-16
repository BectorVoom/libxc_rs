//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 691/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk691(t2062: f64, t7440: f64, t1017: f64, t7351: f64, t142: f64, t2060: f64, t2015: f64, t2029: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7441 = t7440 * t2062;
    let t7442 = 0.5603125e-1_f64 * t7441;
    let t7443 = t7351 * t1017;
    let t7444 = t142 * t7443;
    let t7445 = t2060 * t7444;
    let t7447 = t2015 * t2029;
    (t7441, t7442, t7443, t7444, t7445, t7447)
}
