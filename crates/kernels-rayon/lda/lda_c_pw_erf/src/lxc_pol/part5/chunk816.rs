//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 816/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk816(t574: f64, t7478: f64, t571: f64, t2171: f64, t2566: f64, t3722: f64, t7354: f64, t1459: f64, t519: f64, t3714: f64, t7365: f64, t1485: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7479 = t574 * t7478;
    let t7481 = 8.0_f64 / 15.0_f64 * t571 * t7479;
    let t7483 = 8.0_f64 / 15.0_f64 * t2171 * t2566;
    let t7484 = t3722 * t7354;
    let t7485 = t1459 * t7484;
    let t7487 = 8.0_f64 / 9.0_f64 * t519 * t7485;
    let t7488 = t3714 * t7365;
    let t7489 = t1485 * t7488;
    (t7479, t7481, t7483, t7484, t7485, t7487, t7488, t7489)
}
