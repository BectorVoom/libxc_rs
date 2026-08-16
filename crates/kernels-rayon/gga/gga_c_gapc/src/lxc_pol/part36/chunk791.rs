//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 791/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk791(t2629: f64, t9444: f64, t1081: f64, t2757: f64, t2573: f64, t3303: f64, t1092: f64, t2548: f64, t2562: f64, t327: f64, t8820: f64, t2560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9445 = t9444 * t2629;
    let t9447 = t1081 * t2757;
    let t9449 = t3303 * t2573;
    let t9451 = t1092 * t2548;
    let t9454 = t8820 * t327 * t2562;
    let t9455 = t2560 * t9454;
    (t9445, t9447, t9449, t9451, t9454, t9455)
}
