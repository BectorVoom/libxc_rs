//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1172/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1172<F: Float>(t11821: F, t806: F, t2007: F, t5187: F, t1886: F, t1980: F, t2012: F, t13727: F, t13729: F, t17680: F, t17682: F, t17683: F, t17684: F, t17686: F, t17688: F, t17691: F, t17693: F, t17695: F) -> (F, F, F, F, F, F) {
    let t17697 = 2.0 / 45.0 * t11821 * t806;
    let t17699 = 4.0 / 45.0 * t5187 * t2007;
    let t17702 = 8.0 / 45.0 * t1886 * t1980 * t2012;
    let t17703 = 8.0 / 135.0 * t13727;
    let t17704 = 8.0 / 135.0 * t13729;
    let t17705 = -t17680 + t17682 - t17683 + t17684 + t17686 + t17688 - t17691 + t17693 - t17695 + t17697 + t17699 + t17702 + t17703 + t17704;
    (t17697, t17699, t17702, t17703, t17704, t17705)
}
