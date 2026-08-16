//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1036/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1036(t25374: f64, t25386: f64, t2769: f64, t7056: f64, t1955: f64, t233: f64, t867: f64, t1957: f64, t822: f64) -> (f64, f64, f64, f64, f64) {
    let t25387 = t25386 * t25374;
    let t25390 = t7056 * t2769;
    let t25391 = t1955 * t25390;
    let t25402 = t867 * t233;
    let t25410 = t1957 * t822;
    (t25387, t25390, t25391, t25402, t25410)
}
