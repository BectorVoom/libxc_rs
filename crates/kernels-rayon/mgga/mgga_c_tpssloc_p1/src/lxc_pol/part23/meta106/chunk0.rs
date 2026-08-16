//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 579/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk579(t531: f64, t571: f64, t111: f64, t576: f64, t1406: f64, t604: f64, t1409: f64, t2267: f64, t2274: f64, t2291: f64, t2298: f64, t1441: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3924 = t531 * t571;
    let t3941 = t576 * t111;
    let t3953 = t1406 * t604;
    let t3981 = t2267 * t1409;
    let t3990 = t2274 * t1409;
    let t4007 = t2291 * t1409;
    let t4012 = t2298 * t1409;
    let t4028 = t1441 * t111;
    (t3924, t3941, t3953, t3981, t3990, t4007, t4012, t4028)
}
