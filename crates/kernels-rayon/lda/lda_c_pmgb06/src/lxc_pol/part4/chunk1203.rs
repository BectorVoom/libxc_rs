//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1203/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1203(t130: f64, t830: f64, t5067: f64, t5072: f64, t5137: f64, t5140: f64, t12995: f64, t13020: f64, t15324: f64, t2377: f64, t332: f64, t477: f64) -> (f64, f64, f64, f64, f64) {
    let t15861 = t830 * t130;
    let t15862 = t15861 * t5067;
    let t15864 = 8.0_f64 / 45.0_f64 * t15862 * t5072;
    let t15865 = t15861 * t5137;
    let t15867 = 4.0_f64 / 27.0_f64 * t15865 * t5140;
    let t15870 = 16.0_f64 / 9.0_f64 * t13020 * t12995 * t15324;
    let t15872 = t2377 * t477 * t332;
    (t15862, t15864, t15867, t15870, t15872)
}
