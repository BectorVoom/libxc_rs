//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1152/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1152(t1602: f64, t2549: f64, t2871: f64, t493: f64, t11877: f64, t5336: f64, t4861: f64, t6747: f64, t1447: f64, t6744: f64, t6748: f64, t6791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15173 = 2.0_f64 / 45.0_f64 * t493 * t2871 * t2549 * t1602;
    let t15176 = 4.0_f64 / 45.0_f64 * t493 * t11877 * t5336;
    let t15179 = 4.0_f64 / 15.0_f64 * t493 * t6747 * t4861;
    let t15180 = t1447 * t6744;
    let t15181 = 8.0_f64 / 135.0_f64 * t15180;
    let t15182 = t1447 * t6748;
    let t15183 = 16.0_f64 / 135.0_f64 * t15182;
    let t15184 = t1447 * t6791;
    (t15173, t15176, t15179, t15181, t15183, t15184)
}
