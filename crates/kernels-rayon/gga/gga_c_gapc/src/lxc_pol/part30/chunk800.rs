//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 800/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk800(t3303: f64, t9520: f64, t3300: f64, t7553: f64, t3012: f64, t7557: f64, t2578: f64, t1044: f64, t1055: f64, t311: f64, t1074: f64, t3271: f64, t869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9521 = t3303 * t9520;
    let t9523 = t7553 * t3300;
    let t9525 = t3012 * t7557;
    let t9526 = t2578 * t9525;
    let t9528 = t1055 * t1044;
    let t9529 = t311 * t9528;
    let t9530 = t9529 * t1074;
    let t9532 = t869 * t3271;
    (t9521, t9523, t9526, t9529, t9530, t9532)
}
