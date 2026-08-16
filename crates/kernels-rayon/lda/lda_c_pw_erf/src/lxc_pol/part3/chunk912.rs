//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 912/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk912(t348: f64, t945: f64, t1472: f64, t3864: f64, t3863: f64, t3872: f64, t571: f64, t3802: f64, t3811: f64, t519: f64, t2070: f64, t548: f64, t550: f64) -> (f64, f64, f64, f64, f64) {
    let t9481 = t348 * t945;
    let t9513 = t1472 * t3864;
    let t9540 = t571 * t3863 * t3872;
    let t9590 = t519 * t3802 * t3811;
    let t9593 = t548 * t2070 * t550;
    (t9481, t9513, t9540, t9590, t9593)
}
