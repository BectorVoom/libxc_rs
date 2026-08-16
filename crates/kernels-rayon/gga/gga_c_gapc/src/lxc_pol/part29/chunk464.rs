//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 464/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk464(t1631: f64, t2566: f64, t277: f64, t668: f64, t932: f64, t2546: f64, t786: f64, t2552: f64, t122: f64, t125: f64, t2206: f64, t311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2567 = t2566 * t1631;
    let t2568 = t277 * t2567;
    let t2571 = t932 * t668;
    let t2572 = t2546 * t786;
    let t2573 = t2552 * t2572;
    let t2577 = t2206 * t122 * t125;
    let t2578 = t311 * t2577;
    (t2568, t2571, t2572, t2573, t2577, t2578)
}
