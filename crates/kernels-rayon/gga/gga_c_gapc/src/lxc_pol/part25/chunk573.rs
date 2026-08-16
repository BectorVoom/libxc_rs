//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 573/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk573(t311: f64, t3293: f64, t3297: f64, t2580: f64, t3012: f64, t2578: f64, t191: f64, t932: f64, t2572: f64, t3288: f64, t1068: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3298 = t311 * t3293 * t3297;
    let t3300 = t3012 * t2580;
    let t3301 = t2578 * t3300;
    let t3303 = t932 * t191;
    let t3304 = t3288 * t2572;
    let t3305 = t3303 * t3304;
    let t3307 = t869 * t1068;
    (t3298, t3300, t3301, t3303, t3304, t3305, t3307)
}
