//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 806/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk806(t1084: f64, t8986: f64, t2562: f64, t2636: f64, t8619: f64, t3327: f64, t7191: f64, t2316: f64, t2982: f64, t3391: f64, t2300: f64, t3387: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9414 = t1084 * t8986;
    let t9415 = t2636 * t2562;
    let t9416 = t9414 * t9415;
    let t9418 = t1084 * t8619;
    let t9419 = t3327 * t7191;
    let t9420 = t9418 * t9419;
    let t9422 = t2982 * t2316;
    let t9423 = t3391 * t9422;
    let t9425 = t2982 * t2300;
    let t9426 = t3387 * t9425;
    (t9414, t9415, t9416, t9418, t9419, t9420, t9422, t9423, t9425, t9426)
}
