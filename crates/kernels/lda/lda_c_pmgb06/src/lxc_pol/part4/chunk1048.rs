//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1048/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1048<F: Float>(t13602: F, t13604: F, t15375: F, t15380: F, t15384: F, t15389: F, t15391: F, t15393: F, t15397: F, t15399: F, t15401: F, t15403: F, t12393: F, t13566: F, t13619: F, t13621: F, t13633: F, t13635: F, t13637: F, t13639: F, t13644: F, t15324: F, t15405: F, t15407: F, t15413: F, t15548: F) -> (F, F) {
    let t15626 = -0.07111111111111111 * t13602 - 0.017777777777777778 * t13604 + 0.14396666666666666 * t15375 + 0.47988888888888886 * t15380 - 0.03999074074074074 * t15384 - 0.10664197530864197 * t15389 + 0.14396666666666666 * t15391 - 1.0557555555555556 * t15393 - 0.21595 * t15397 - 0.047988888888888886 * t15399 + 0.015996296296296297 * t15401 - 0.09597777777777777 * t15403;
    let t15641 = 0.026660493827160493 * t15405 + 0.3519185185185185 * t15407 - 0.03999074074074074 * t15413 - 0.023703703703703703 * t15548 * t13566 * t15324 + 0.05925925925925926 * t13619 - 0.009876543209876543 * t13621 + 0.002962962962962963 * t13633 + 0.003950617283950617 * t13635 + 0.011851851851851851 * t13637 - 0.017777777777777778 * t13639 + 0.05333333333333334 * t13644 - 0.09597777777777777 * t12393;
    (t15626, t15641)
}
