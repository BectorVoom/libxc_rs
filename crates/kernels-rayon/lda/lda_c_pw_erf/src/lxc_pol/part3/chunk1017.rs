//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1017/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1017(t11907: f64, t11909: f64, t4506: f64, t219: f64, t4048: f64, t3589: f64, t9234: f64, t9244: f64, t9246: f64, t9251: f64, t1506: f64, t184: f64, t494: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11912 = 8.0_f64 / 3.0_f64 * t4506 * t11907 * t11909;
    let t11913 = t4048 * t219;
    let t11914 = t11913 * t3589;
    let t11917 = 32.0_f64 / 27.0_f64 * t4506 * t11914 * t11909;
    let t11918 = 8.0_f64 / 15.0_f64 * t9234;
    let t11919 = 8.0_f64 / 45.0_f64 * t9244;
    let t11920 = 16.0_f64 / 135.0_f64 * t9246;
    let t11921 = 4.0_f64 / 45.0_f64 * t9251;
    let t11925 = 4.0_f64 / 5.0_f64 * t494 * t1506 * t184 * t786;
    (t11912, t11914, t11917, t11918, t11919, t11920, t11921, t11925)
}
