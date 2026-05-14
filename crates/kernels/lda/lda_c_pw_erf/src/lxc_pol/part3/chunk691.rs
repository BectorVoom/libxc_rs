//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 691/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk691<F: Float>(t1319: F, t4780: F, t1318: F, t1954: F, t549: F, t4758: F, t2010: F, t3863: F, t571: F, t1949: F, t3854: F, t219: F, t4062: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4781 = t1319 * t4780;
    let t4783 = 8.0 / 45.0 * t1318 * t4781;
    let t4784 = t1954 * t549;
    let t4785 = t4758 * t4784;
    let t4787 = 32.0 / 45.0 * t1318 * t4785;
    let t4788 = t3863 * t2010;
    let t4790 = 16.0 / 135.0 * t571 * t4788;
    let t4791 = t3854 * t1949;
    let t4793 = 32.0 / 135.0 * t571 * t4791;
    let t4794 = t4062 * t219;
    (t4781, t4783, t4784, t4785, t4787, t4788, t4790, t4791, t4793, t4794)
}
