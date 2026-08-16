//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 328/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk328(t101: f64, t995: f64, t115: f64, t594: f64, t653: f64, t128: f64, t144: f64) -> (f64, f64, f64) {
    let t1412 = t995 * t101;
    let t1413 = t1412 * t115;
    let t1414 = t594 * t653;
    let t1415 = t1413 * t1414;
    let t1416 = t128 * t144;
    (t1412, t1415, t1416)
}
