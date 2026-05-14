//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1202/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1202<F: Float>(t17768: F, t3794: F, t6917: F, t13391: F, t13397: F, t17736: F, t17739: F, t17741: F, t17743: F, t17745: F, t17747: F, t17751: F, t17754: F, t17756: F, t17758: F, t17763: F, t17767: F) -> (F, F, F, F, F) {
    let t17769 = 16.0 / 135.0 * t17768;
    let t17771 = 16.0 / 15.0 * t3794 * t6917;
    let t17772 = 16.0 / 45.0 * t13391;
    let t17773 = 16.0 / 45.0 * t13397;
    let t17774 = t17736 + t17739 - t17741 + t17743 + t17745 + t17747 - t17751 + t17754 - t17756 - t17758 - t17763 - t17767 + t17769 - t17771 + t17772 + t17773;
    (t17769, t17771, t17772, t17773, t17774)
}
