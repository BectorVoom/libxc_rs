//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1097/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1097(t1845: f64, t186: f64, t1803: f64, t1648: f64, t623: f64, t1603: f64, t19422: f64, t137: f64, t1509: f64, t1839: f64, t442: f64, t5214: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19844 = t1845 * t186;
    let t19902 = t1803 * t186;
    let t19916 = t1648 * t623;
    let t20171 = t19422 * t1603;
    let t20198 = t1509 * t137;
    let t20200 = t5214 * t1839 * t20198 * t442;
    (t19844, t19902, t19916, t20171, t20198, t20200)
}
