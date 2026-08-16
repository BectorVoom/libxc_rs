//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1088/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1088(t4103: f64, t872: f64, t132: f64, t435: f64, t4978: f64, t5040: f64, t4974: f64, t432: f64, t5326: f64, t486: f64, t5044: f64, t1554: f64, t161: f64, t1836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12804 = t872 * t4103;
    let t12807 = t132 * t435 * t4978;
    let t12816 = t132 * t435 * t5040;
    let t12822 = t132 * t435 * t4974;
    let t12825 = t432 * t5326;
    let t12828 = t486 * t5044;
    let t12831 = t161 * t1554 * t1836;
    (t12804, t12807, t12816, t12822, t12825, t12828, t12831)
}
