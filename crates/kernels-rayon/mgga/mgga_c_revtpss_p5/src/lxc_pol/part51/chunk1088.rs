//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1088/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1088(t1518: f64, t6982: f64, t1936: f64, t1931: f64, t4292: f64, t33602: f64, t7002: f64, t25805: f64, t7741: f64, t28025: f64, t28042: f64, t6985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t125362 = t6982 * t1518;
    let t125363 = t125362 * t1936;
    let t125365 = t1931 * t4292;
    let t125366 = t125365 * t1936;
    let t125368 = t33602 * t7002;
    let t125370 = t25805 * t7741;
    let t125372 = t28025 * t7741;
    let t125374 = t6985 * t28042;
    (t125362, t125363, t125365, t125366, t125368, t125370, t125372, t125374)
}
