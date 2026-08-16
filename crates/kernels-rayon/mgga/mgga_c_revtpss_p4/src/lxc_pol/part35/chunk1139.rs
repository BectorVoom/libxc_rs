//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1139/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1139(t60224: f64, t7342: f64, t13272: f64, t26178: f64, t26205: f64, t7709: f64, t7702: f64, t1923: f64, t26204: f64, t7719: f64, t28150: f64, t7348: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t101785 = t60224 * t7342;
    let t101788 = t13272 * t26178;
    let t101793 = t7709 * t26205;
    let t101907 = t7702 * t26205;
    let t101929 = t1923 * t26204 * t7719;
    let t101970 = t7348 * t28150;
    (t101785, t101788, t101793, t101907, t101929, t101970)
}
