//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 810/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk810(t19: f64, t761: f64, t3114: f64, t3440: f64, t2200: f64, t3439: f64, t3438: f64, t3103: f64, t885: f64, t3379: f64, t2520: f64, t2972: f64) -> (f64, f64, f64, f64, f64) {
    let t9651 = t761 * t19;
    let t9652 = t9651 * t3114;
    let t9653 = t9652 * t3440;
    let t9655 = t2200 * t3439;
    let t9656 = t3438 * t9655;
    let t9658 = t885 * t3103;
    let t9659 = t9658 * t3379;
    let t9661 = t2520 * t2972;
    (t9652, t9653, t9656, t9659, t9661)
}
