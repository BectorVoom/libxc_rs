//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 726/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk726(t2130: f64, t4567: f64, t493: f64, t1518: f64, t812: f64, t548: f64, t219: f64, t573: f64, t558: f64, t2070: f64, t211: f64, t1524: f64, t835: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4568 = t4567 * t2130;
    let t4569 = t493 * t4568;
    let t4570 = 4.0_f64 / 9.0_f64 * t4569;
    let t4571 = t1518 * t812;
    let t4572 = t548 * t4571;
    let t4573 = 8.0_f64 / 135.0_f64 * t4572;
    let t4574 = t573 * t219;
    let t4575 = t4574 * t558;
    let t4576 = t2070 * t4575;
    let t4578 = 8.0_f64 / 45.0_f64 * t211 * t4576;
    let t4580 = 4.0_f64 / 15.0_f64 * t1524 * t835;
    (t4568, t4570, t4571, t4573, t4574, t4575, t4576, t4578, t4580)
}
