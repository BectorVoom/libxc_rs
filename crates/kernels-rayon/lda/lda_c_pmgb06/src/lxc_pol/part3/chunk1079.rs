//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1079/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1079(t132: f64, t435: f64, t4974: f64, t9644: f64, t432: f64, t5326: f64, t9754: f64, t486: f64, t5044: f64, t1554: f64, t161: f64, t1836: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12822 = t132 * t435 * t4974;
    let t12823 = 2.0_f64 / 15.0_f64 * t12822;
    let t12824 = 2.0_f64 / 15.0_f64 * t9644;
    let t12825 = t432 * t5326;
    let t12826 = 2.0_f64 / 15.0_f64 * t12825;
    let t12827 = 2.0_f64 / 15.0_f64 * t9754;
    let t12828 = t486 * t5044;
    let t12829 = t12828 / 45.0_f64;
    let t12831 = t161 * t1554 * t1836;
    (t12823, t12824, t12826, t12827, t12829, t12831)
}
