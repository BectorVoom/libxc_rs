//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3166/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3166(t1012: f64, t44958: f64, t13026: f64, t140: f64, t1222: f64, t16715: f64, t1224: f64, t5052: f64, t697: f64, t12915: f64, t17344: f64, t17345: f64, t247: f64) -> (f64, f64, f64, f64, f64) {
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    let t57486 = t1222 * t57484 * t16715;
    let t57490 = t1222 * t697 * t1224 * t5052;
    let t57508 = t17344 * t247 * t12915 * t17345;
    (t57480, t57484, t57486, t57490, t57508)
}
