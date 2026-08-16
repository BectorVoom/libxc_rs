//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1358/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1358(t2966: f64, t307: f64, t302: f64, t11132: f64, t11337: f64, t944: f64, t2969: f64, t310: f64, t2979: f64, t964: f64, t3011: f64, t960: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11408 = 1.0_f64 / t2966 / t307;
    let t11409 = t302 * t11408;
    let t11422 = 0.16068111111111111111e1_f64 * t11132;
    let t11423 = 0.46308888888888888888e0_f64 * t11337;
    let t11449 = 1.0_f64 / t2966 / t944;
    let t11450 = t302 * t11449;
    let t11452 = 1.0_f64 / t2969 / t310;
    let t11456 = t2979 * t964;
    let t11461 = t960 * t3011;
    (t11409, t11422, t11423, t11450, t11452, t11456, t11461)
}
