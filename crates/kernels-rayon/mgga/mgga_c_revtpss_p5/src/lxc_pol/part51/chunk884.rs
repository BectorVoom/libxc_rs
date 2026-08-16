//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 884/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk884(t2007: f64, t4292: f64, t670: f64, t7883: f64, t1843: f64, t7002: f64, t651: f64, t2322: f64, t7742: f64, t4254: f64, t1310: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28050 = t2007 * t4292;
    let t28053 = t7883 * t670;
    let t28056 = t1843 * t7002;
    let t28058 = 2.0_f64 * t651 * t28056;
    let t28060 = 2.0_f64 * t2322 * t7742;
    let t28062 = 2.0_f64 * t4254 * t7742;
    let t28063 = t1310 * t7741;
    (t28050, t28053, t28056, t28058, t28060, t28062, t28063)
}
