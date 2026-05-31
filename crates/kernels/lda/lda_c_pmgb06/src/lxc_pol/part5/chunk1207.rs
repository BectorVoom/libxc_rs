//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1207/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1207<F: Float>(t638: F, t7414: F, t643: F, t11166: F, t11169: F, t11175: F, t11177: F, t11178: F, t11180: F, t11183: F, t11184: F, t15045: F, t15054: F, t8837: F, t8841: F, t8844: F, t8853: F, t9037: F) -> F {
    let t21820 = t638 * t7414;
    let t21822 = t643 * t7414;
    let t21824 = -F::cast_from(3076.205657464922_f64) * t11166 - t11169 + t11175 + F::cast_from(3.0_f64) * t11177 - t8837 + F::cast_from(60.0_f64) * t15045 + t8841 - F::cast_from(1.7544670867903938_f64) * t11178 - F::cast_from(10.526802520742363_f64) * t11180 - F::cast_from(24.0_f64) * t8844 - t11183 - t11184 - F::cast_from(36.0_f64) * t15054 - t8853 + t9037 + F::cast_from(4.0_f64) * t21820 - F::cast_from(4.0_f64) * t21822;
    t21824
}
