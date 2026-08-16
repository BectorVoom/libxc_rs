//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 826/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk826(t2405: f64, t2982: f64, t3391: f64, t197: f64, t7975: f64, t1077: f64, t2493: f64, t3096: f64, t3430: f64, t154: f64, t7073: f64, t2580: f64, t8686: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9635 = t2982 * t2405;
    let t9636 = t3391 * t9635;
    let t9638 = t197 * t7975;
    let t9639 = t1077 * t9638;
    let t9641 = t3096 * t2493;
    let t9642 = t3430 * t9641;
    let t9644 = t7073 * t154;
    let t9645 = t8686 * t2580;
    (t9635, t9636, t9638, t9639, t9642, t9644, t9645)
}
