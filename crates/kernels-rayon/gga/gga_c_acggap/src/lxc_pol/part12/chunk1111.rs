//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1111/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1111(t5277: f64, t7561: f64, t1181: f64, t4665: f64, t7351: f64, t7564: f64, t30219: f64, t8469: f64, t4752: f64, t604: f64, t7575: f64, t4762: f64) -> (f64, f64, f64, f64, f64) {
    let t35768 = t7561 * t5277;
    let t35772 = t7564 * t1181 * t7351 * t4665;
    let t35774 = t30219 * t8469;
    let t35778 = t7575 * t1181 * t604 * t4752;
    let t35782 = t7564 * t1181 * t7351 * t4762;
    (t35768, t35772, t35774, t35778, t35782)
}
