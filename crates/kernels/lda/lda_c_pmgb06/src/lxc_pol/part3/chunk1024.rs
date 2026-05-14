//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1024/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1024<F: Float>(t3050: F, t802: F, t405: F, t4892: F, t4889: F, t4902: F, t4906: F, t4913: F, t1464: F, t524: F, t2911: F, t3357: F, t103: F, t11991: F, t12547: F, t12558: F, t12584: F, t12588: F, t12594: F, t12599: F, t12605: F, t12864: F, t13330: F, t13332: F, t13335: F, t13337: F, t13340: F, t13565: F, t1576: F, t2060: F, t3358: F, t525: F, t9967: F) -> (F, F) {
    let t14068 = t802 * t3050;
    let t14073 = t405 * t4892;
    let t14078 = t405 * t4889;
    let t14080 = t405 * t4902;
    let t14082 = t4913 * t4906;
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    let t14115 = -0.08 * t103 * t1576 * t11991 + 0.08 * t14073 + 0.16 * t103 * t525 * t12864 - 0.02666666666666667 * t14078 + 0.005925925925925926 * t14080 - 0.057777777777777775 * t14082 - 0.0022222222222222222 * t103 * t1576 * t12584 + 0.013333333333333334 * t2060 * t1576 * t12588 - 0.006913580246913581 * t103 * t9967 * t12594 + 0.017777777777777778 * t2060 * t3358 * t12599 + 0.013333333333333334 * t103 * t525 * t12605 - 0.08 * t2060 * t525 * t12558 - 0.8638 * t13330 + 0.21595 * t13332 + 0.8638 * t13335 - 0.5278777777777778 * t13337 - 0.12 * t13565 * t14106 * t12547 - 0.008888888888888889 * t13565 * t14110 * t12547 + 0.47988888888888886 * t13340;
    (t14068, t14115)
}
