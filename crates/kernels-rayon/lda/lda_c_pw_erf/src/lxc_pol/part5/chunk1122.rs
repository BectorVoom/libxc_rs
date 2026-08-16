//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1122/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1122(t10030: f64, t7752: f64, t18184: f64, t2010: f64, t3974: f64, t1949: f64, t2478: f64, t4574: f64, t1944: f64, t5165: f64, t12314: f64, t6725: f64) -> (f64, f64, f64, f64, f64) {
    let t20836 = t10030 * t7752;
    let t20837 = 32.0_f64 / 45.0_f64 * t20836;
    let t20840 = 8.0_f64 / 15.0_f64 * t3974 * t18184 * t2010;
    let t20844 = 16.0_f64 / 15.0_f64 * t3974 * t4574 * t2478 * t1949;
    let t20848 = 8.0_f64 / 9.0_f64 * t3974 * t5165 * t2478 * t1944;
    let t20850 = 16.0_f64 / 15.0_f64 * t12314 * t6725;
    (t20837, t20840, t20844, t20848, t20850)
}
