//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 827/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk827(t9644: f64, t9645: f64, t1026: f64, t2497: f64, t334: f64, t19: f64, t761: f64, t3114: f64, t3440: f64, t2200: f64, t3439: f64, t3438: f64) -> (f64, f64, f64, f64, f64) {
    let t9646 = t9644 * t9645;
    let t9648 = t2497 * t1026;
    let t9649 = t9648 * t334;
    let t9651 = t761 * t19;
    let t9652 = t9651 * t3114;
    let t9653 = t9652 * t3440;
    let t9655 = t2200 * t3439;
    let t9656 = t3438 * t9655;
    (t9646, t9649, t9652, t9653, t9656)
}
