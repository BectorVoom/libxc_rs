//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1147/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1147(t2389: f64, t3388: f64, t3750: f64, t11862: f64, t9425: f64, t1033: f64, t188: f64, t2480: f64, t277: f64, t333: f64, t311: f64, t3273: f64, t34081: f64) -> (f64, f64, f64, f64, f64) {
    let t34154 = t2389 * t3750 * t3388;
    let t34156 = t11862 * t9425;
    let t34159 = t1033 * t188;
    let t34161 = t277 * t2480 * t34159 * t333;
    let t34164 = t311 * t34081 * t3273;
    (t34154, t34156, t34159, t34161, t34164)
}
