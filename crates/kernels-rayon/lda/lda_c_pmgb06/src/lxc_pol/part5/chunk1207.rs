//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1207/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1207(t638: f64, t7414: f64, t643: f64, t11166: f64, t11169: f64, t11175: f64, t11177: f64, t11178: f64, t11180: f64, t11183: f64, t11184: f64, t15045: f64, t15054: f64, t8837: f64, t8841: f64, t8844: f64, t8853: f64, t9037: f64) -> f64 {
    let t21820 = t638 * t7414;
    let t21822 = t643 * t7414;
    let t21824 = -3076.205657464922_f64 * t11166 - t11169 + t11175 + 3.0_f64 * t11177 - t8837 + 60.0_f64 * t15045 + t8841 - 1.7544670867903938_f64 * t11178 - 10.526802520742363_f64 * t11180 - 24.0_f64 * t8844 - t11183 - t11184 - 36.0_f64 * t15054 - t8853 + t9037 + 4.0_f64 * t21820 - 4.0_f64 * t21822;
    t21824
}
