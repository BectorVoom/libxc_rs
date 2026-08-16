//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 718/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk718(t1266: f64, t996: f64, t1001: f64, t2902: f64, t632: f64, t458: f64, t998: f64, t568: f64, t2903: f64, t1587: f64, t493: f64, t2911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8521 = t996 * t1266;
    let t8522 = t8521 * t1001;
    let t8524 = t2902 * t632;
    let t8525 = t998 * t458;
    let t8526 = t8524 * t8525;
    let t8528 = t998 * t568;
    let t8529 = t2903 * t8528;
    let t8531 = t493 * t1587;
    let t8532 = t2911 * t8531;
    (t8521, t8522, t8524, t8526, t8529, t8532)
}
