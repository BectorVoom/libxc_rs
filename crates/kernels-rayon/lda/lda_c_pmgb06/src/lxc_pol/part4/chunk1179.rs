//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1179/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1179(t1420: f64, t6495: f64, t1444: f64, t6761: f64, t493: f64, t5447: f64, t6760: f64, t1414: f64, t337: f64, t5974: f64, t1915: f64, t2948: f64, t439: f64, t6774: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15496 = 8.0_f64 / 45.0_f64 * t1420 * t6495;
    let t15498 = 4.0_f64 / 45.0_f64 * t1444 * t6761;
    let t15501 = 4.0_f64 / 45.0_f64 * t493 * t5447 * t6760;
    let t15503 = t1414 * t5974 * t337;
    let t15506 = 4.0_f64 / 45.0_f64 * t493 * t1915 * t15503;
    let t15509 = 2.0_f64 / 45.0_f64 * t439 * t2948 * t6774;
    (t15496, t15498, t15501, t15503, t15506, t15509)
}
