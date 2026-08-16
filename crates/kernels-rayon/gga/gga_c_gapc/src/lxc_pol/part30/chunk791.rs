//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 791/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk791(t2600: f64, t9408: f64, t8769: f64, t933: f64, t2629: f64, t1084: f64, t8986: f64, t2562: f64, t2636: f64, t8619: f64, t3327: f64, t7191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9409 = t9408 * t2600;
    let t9411 = t933 * t8769;
    let t9412 = t9411 * t2629;
    let t9414 = t1084 * t8986;
    let t9415 = t2636 * t2562;
    let t9416 = t9414 * t9415;
    let t9418 = t1084 * t8619;
    let t9419 = t3327 * t7191;
    (t9409, t9412, t9414, t9415, t9416, t9418, t9419)
}
