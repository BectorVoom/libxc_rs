//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 678/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk678<F: Float>(t534: F, t6446: F, t515: F, t1788: F, t513: F, t13: F, t1791: F, t30: F, t6434: F, t1809: F, t1820: F, t1826: F, t1865: F, t3648: F, t4: F, t566: F, t571: F, t573: F, t581: F, t6343: F, t6348: F, t6356: F, t6359: F, t6383: F, t6388: F, t6392: F, t6395: F, t6400: F, t6401: F, t6406: F, t6408: F, t6420: F, t6425: F, t6428: F, t6437: F, t84: F) -> (F, F, F, F, F, F, F, F) {
    let t6447 = t6446 * t534;
    let t6449 = F::new(1.0) * t515 * t6447;
    let t6451 = F::new(1.0) / t1788 / t513;
    let t6452 = t13 * t6451;
    let t6454 = F::new(1.0) / t1791 / t30;
    let t6455 = t6434 * t6454;
    let t6457 = F::new(0.51725014705706168417e3) * t6452 * t6455;
    let t6461 = F::new(0.96494049533612093922e2) * t1826 * t6343 * t571 + F::new(0.51947267698127589897e2) * t1865 * t6348 + F::new(0.56969282336565386482e-3) * t4 * t3648 * t84 + t6356 - t6359 + F::new(1.0) * t566 * t6383 + F::new(0.20691336878655965246e4) * t6388 * t6392 + F::new(6.0) * t1826 * t6395 - F::new(0.19298809906722418785e3) * t6400 * t6401 - F::new(0.1038945353962551798e3) * t6406 * t6408 + F::new(0.58482233974552040708e0) * t581 * t6420 + F::new(0.1025389702100779493e4) * t6425 * t6428 + t6437 - t6449 - t6457 - F::new(6.0) * t1809 * t573 * t1820;
    (t6447, t6449, t6451, t6452, t6454, t6455, t6457, t6461)
}
