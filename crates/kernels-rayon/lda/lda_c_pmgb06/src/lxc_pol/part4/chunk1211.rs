//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1211/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1211(t4602: f64, t6395: f64, t2496: f64, t493: f64, t9925: f64, t2979: f64, t6390: f64, t10152: f64, t6517: f64, t1908: f64, t5187: f64, t1420: f64, t6524: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15962 = 8.0_f64 / 45.0_f64 * t4602 * t6395;
    let t15965 = 2.0_f64 / 45.0_f64 * t493 * t9925 * t2496;
    let t15968 = 4.0_f64 / 45.0_f64 * t493 * t2979 * t6390;
    let t15971 = 4.0_f64 / 45.0_f64 * t493 * t10152 * t6517;
    let t15973 = 4.0_f64 / 45.0_f64 * t5187 * t1908;
    let t15975 = 4.0_f64 / 45.0_f64 * t1420 * t6524;
    (t15962, t15965, t15968, t15971, t15973, t15975)
}
