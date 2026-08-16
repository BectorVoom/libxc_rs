//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 795/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk795(t25374: f64, t25386: f64, t25378: f64, t2769: f64, t7056: f64, t1955: f64, t1949: f64, t822: f64, t1950: f64, t867: f64, t786: f64, t2467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25387 = t25386 * t25374;
    let t25388 = t25387 * t25378;
    let t25390 = t7056 * t2769;
    let t25391 = t1955 * t25390;
    let t25392 = t822 * t1949;
    let t25398 = t1950 * t867;
    let t25399 = t786 * t25398;
    let t25400 = t25399 * t2467;
    (t25387, t25388, t25391, t25392, t25399, t25400)
}
