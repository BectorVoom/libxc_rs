//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 753/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk753(t611: f64, t8769: f64, t5409: f64, t204: f64, t474: f64, t1970: f64, t1266: f64, t191: f64, t1046: f64, t5730: f64, t599: f64, t596: f64) -> (f64, f64, f64, f64) {
    let t8921 = t611 * t8769;
    let t8922 = t8921 * t5409;
    let t8926 = t474 * t204;
    let t8927 = t1970 * t8926;
    let t8929 = t1266 * t191;
    let t8930 = t8929 * t1046;
    let t8932 = t5730 * t599;
    let t8933 = t596 * t8932;
    (t8922, t8927, t8930, t8933)
}
