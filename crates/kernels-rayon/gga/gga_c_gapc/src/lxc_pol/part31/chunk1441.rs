//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1441/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1441(t12650: f64, t2464: f64, t2469: f64, t2470: f64, t36119: f64, t36266: f64, t36269: f64, t36270: f64, t36271: f64, t36272: f64, t36275: f64, t36280: f64, t36283: f64, t36285: f64, t36288: f64, t36290: f64, t36293: f64, t36295: f64, t38692: f64, t3914: f64, t7063: f64, t972: f64) -> f64 {
    let t38825 = 4.0_f64 * t12650 * t2469 * t972 - 6.0_f64 * t2470 * t3914 * t7063 - 2.0_f64 * t12650 * t2464 - t36119 + t36266 - t36269 + t36270 + t36271 - t36272 + t36275 + t36280 - t36283 + t36285 - t36288 - t36290 - t36293 - t36295 - t38692;
    t38825
}
