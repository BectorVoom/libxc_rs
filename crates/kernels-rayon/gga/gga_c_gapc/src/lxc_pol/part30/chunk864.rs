//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 864/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk864(t10316: f64, t3230: f64, t2225: f64, t3198: f64, t2217: f64, t10203: f64, t2456: f64, t3258: f64, t3253: f64, t6948: f64, t10293: f64, t6951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10325 = t10203 * t2456;
    let t10326 = t3258 * t10325;
    let t10328 = t3253 * t6948;
    let t10329 = t10293 * t6951;
    (t10317, t10319, t10321, t10326, t10328, t10329)
}
