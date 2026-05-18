//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1348/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1348<F: Float>(t13729: F, t17680: F, t17682: F, t17683: F, t17684: F, t17686: F, t17688: F, t17691: F, t17693: F, t17695: F, t17697: F, t17699: F, t17702: F, t17703: F) -> (F, F) {
    let t17704 = F::new(8.0) / F::new(135.0) * t13729;
    let t17705 = -t17680 + t17682 - t17683 + t17684 + t17686 + t17688 - t17691 + t17693 - t17695 + t17697 + t17699 + t17702 + t17703 + t17704;
    (t17704, t17705)
}
