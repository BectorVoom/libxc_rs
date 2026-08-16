//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 472/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk472<F: Float>(t169: F, t301: F, t717: F, t865: F, t1045: F, t1066: F, t1069: F, t1767: F, t1774: F, t1776: F, t1778: F, t1780: F, t1800: F, t1801: F, t910: F, t916: F, t938: F) -> (F, F) {
    let t1885 = t169 * t717 * t865 * t301;
    let t1887 = t910 - t916 + t938 - t1767 - t1774 + t1776 + t1778 - t1780 + t1800 + t1066 - t1069 - t1801 - t1045;
    (t1885, t1887)
}
