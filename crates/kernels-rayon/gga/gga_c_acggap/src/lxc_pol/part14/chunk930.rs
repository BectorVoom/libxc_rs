//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 930/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk930(t3919: f64, t7948: f64, t3035: f64, t3923: f64, t609: f64, t30028: f64, t315: f64, t323: f64, t3242: f64, t7927: f64, t872: f64, t2130: f64, t3874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32082 = t7948 * t3919;
    let t32091 = 0.39512695097613069591e1_f64 * t3035 * t609 * t3923;
    let t32092 = t315 * t30028;
    let t32109 = 0.19756347548806534796e1_f64 * t3242 * t609 * t323;
    let t32121 = t7927 * t872;
    let t32123 = t2130 * t3874;
    (t32082, t32091, t32092, t32109, t32121, t32123)
}
