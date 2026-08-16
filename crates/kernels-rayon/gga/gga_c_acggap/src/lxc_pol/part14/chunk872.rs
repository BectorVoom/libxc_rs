//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 872/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk872(t30318: f64, t438: f64, t2092: f64, t7610: f64, t1165: f64, t30209: f64, t3655: f64, t7351: f64, t12935: f64, t7336: f64, t1181: f64, t3355: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t30319 = t30318 * t438;
    let t30321 = t7610 * t2092;
    let t30325 = t30209 * t1165 * t7351 * t3655;
    let t30327 = t12935 * t7336;
    let t30330 = t30327 * t1181 * t599 * t3355;
    (t30319, t30321, t30325, t30327, t30330)
}
