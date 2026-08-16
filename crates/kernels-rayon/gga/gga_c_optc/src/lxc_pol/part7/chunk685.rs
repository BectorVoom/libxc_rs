//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 685/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk685(t534: f64, t6446: f64, t515: f64, t1788: f64, t513: f64, t13: f64, t1791: f64, t30: f64, t6434: f64, t1809: f64, t1820: f64, t1826: f64, t1865: f64, t3648: f64, t4: f64, t566: f64, t571: f64, t573: f64, t581: f64, t6343: f64, t6348: f64, t6356: f64, t6359: f64, t6383: f64, t6388: f64, t6392: f64, t6395: f64, t6400: f64, t6401: f64, t6406: f64, t6408: f64, t6420: f64, t6425: f64, t6428: f64, t6437: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6447 = t6446 * t534;
    let t6449 = 1.0_f64 * t515 * t6447;
    let t6451 = 1.0_f64 / t1788 / t513;
    let t6452 = t13 * t6451;
    let t6454 = 1.0_f64 / t1791 / t30;
    let t6455 = t6434 * t6454;
    let t6457 = 0.51725014705706168417e3_f64 * t6452 * t6455;
    let t6461 = 0.96494049533612093922e2_f64 * t1826 * t6343 * t571 + 0.51947267698127589897e2_f64 * t1865 * t6348 + 0.56969282336565386482e-3_f64 * t4 * t3648 * t84 + t6356 - t6359 + 1.0_f64 * t566 * t6383 + 0.20691336878655965246e4_f64 * t6388 * t6392 + 6.0_f64 * t1826 * t6395 - 0.19298809906722418785e3_f64 * t6400 * t6401 - 0.1038945353962551798e3_f64 * t6406 * t6408 + 0.58482233974552040708e0_f64 * t581 * t6420 + 0.1025389702100779493e4_f64 * t6425 * t6428 + t6437 - t6449 - t6457 - 6.0_f64 * t1809 * t573 * t1820;
    (t6447, t6449, t6451, t6452, t6454, t6455, t6457, t6461)
}
