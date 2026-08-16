//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1030/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1030(t32243: f64, t32295: f64, t532: f64, t1450: f64, t2014: f64, t2322: f64, t8457: f64, t1937: f64, t25805: f64, t28025: f64, t6985: f64, t6993: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32296 = t32243 + t32295;
    let t32297 = t532 * t32296;
    let t32298 = t32297 * t1450;
    let t32299 = t2014 * t32298;
    let t32301 = t2322 * t8457;
    let t32303 = t25805 * t1937;
    let t32305 = t28025 * t1937;
    let t32307 = t6985 * t6993;
    (t32296, t32297, t32298, t32299, t32301, t32303, t32305, t32307)
}
