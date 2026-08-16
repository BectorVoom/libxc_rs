//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1186/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1186<F: Float>(t12393: F, t13566: F, t13619: F, t13621: F, t13633: F, t13635: F, t13637: F, t13639: F, t13644: F, t15324: F, t15405: F, t15407: F, t15413: F, t15548: F) -> F {
    let t15641 = F::cast_from(0.026660493827160493_f64) * t15405 + F::cast_from(0.3519185185185185_f64) * t15407 - F::cast_from(0.03999074074074074_f64) * t15413 - F::cast_from(0.023703703703703703_f64) * t15548 * t13566 * t15324 + F::cast_from(0.05925925925925926_f64) * t13619 - F::cast_from(0.009876543209876543_f64) * t13621 + F::cast_from(0.002962962962962963_f64) * t13633 + F::cast_from(0.003950617283950617_f64) * t13635 + F::cast_from(0.011851851851851851_f64) * t13637 - F::cast_from(0.017777777777777778_f64) * t13639 + F::cast_from(0.05333333333333334_f64) * t13644 - F::cast_from(0.09597777777777777_f64) * t12393;
    t15641
}
