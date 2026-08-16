//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 521/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk521(t2602: f64, t322: f64, t2377: f64, t2380: f64, t2412: f64, t2421: f64, t2430: f64, t2443: f64, t2451: f64, t2470: f64, t2479: f64, t2542: f64, t2561: f64) -> (f64, f64) {
    let t2603 = t322 * t2602;
    let t2606 = -t2451 + t2380 - t2377 + t2412 + t2421 + t2542 + t2430 - t2443 + t2561 - t2470 - t2479;
    (t2603, t2606)
}
