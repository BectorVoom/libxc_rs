//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 628/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk628(t211: f64, t4561: f64, t1518: f64, t785: f64, t493: f64, t1: f64, t1124: f64, t2130: f64, t812: f64, t548: f64, t219: f64, t573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4562 = t211 * t4561;
    let t4564 = t1518 * t785;
    let t4565 = t493 * t4564;
    let t4567 = t1 * t1124;
    let t4568 = t4567 * t2130;
    let t4569 = t493 * t4568;
    let t4571 = t1518 * t812;
    let t4572 = t548 * t4571;
    let t4574 = t573 * t219;
    (t4562, t4564, t4565, t4567, t4568, t4569, t4571, t4572, t4574)
}
