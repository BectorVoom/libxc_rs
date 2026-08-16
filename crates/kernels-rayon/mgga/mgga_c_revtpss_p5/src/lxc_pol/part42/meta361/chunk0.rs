//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1176/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1176(t1222: f64, t17472: f64, t1012: f64, t13026: f64, t1263: f64, t5245: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t3172: f64, t5286: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17474 = t1222 * t17472 / 324.0_f64;
    let t17475 = t1012 * t13026;
    let t17500 = t1263 * t5245;
    let t17505 = t1234 * t5390;
    let t17509 = 0.15244095330869239812e-2_f64 * t5293 * t3704;
    let t17544 = t3172 * t5286;
    (t17474, t17475, t17500, t17505, t17509, t17544)
}
