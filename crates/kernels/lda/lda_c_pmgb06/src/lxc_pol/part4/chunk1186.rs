//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1186/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1186<F: Float>(t12393: F, t13566: F, t13619: F, t13621: F, t13633: F, t13635: F, t13637: F, t13639: F, t13644: F, t15324: F, t15405: F, t15407: F, t15413: F, t15548: F) -> F {
    let t15641 = F::new(0.026660493827160493) * t15405 + F::new(0.3519185185185185) * t15407 - F::new(0.03999074074074074) * t15413 - F::new(0.023703703703703703) * t15548 * t13566 * t15324 + F::new(0.05925925925925926) * t13619 - F::new(0.009876543209876543) * t13621 + F::new(0.002962962962962963) * t13633 + F::new(0.003950617283950617) * t13635 + F::new(0.011851851851851851) * t13637 - F::new(0.017777777777777778) * t13639 + F::new(0.05333333333333334) * t13644 - F::new(0.09597777777777777) * t12393;
    t15641
}
