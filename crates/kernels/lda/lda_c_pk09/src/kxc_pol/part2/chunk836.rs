//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 836/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk836<F: Float>(t5043: F, t5056: F, t5158: F, t5166: F, t5177: F, t5193: F, t5520: F, t5536: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t9948: F, t9952: F, t9956: F, t9959: F) -> (F,) {
    let t9972 = 0.22687409291590604 * t9948 + 0.22687409291590604 * t9952 - 0.22687409291590604 * t9956 + 0.15124939527727072 * t9959 - 0.04525483399593904 * t9623 - 0.015084944665313014 * t9631 - 0.04525483399593904 * t9635 - 0.04525483399593904 * t9742 - 0.04525483399593904 * t9750 - 0.04525483399593904 * t5043 - 0.015084944665313014 * t5056 + t5520 - 0.15124939527727072 * t5177 + 0.15124939527727072 * t5193 + t5536 - 0.4537481858318121 * t5158 + 0.4537481858318121 * t5166;
    (t9972,)
}
