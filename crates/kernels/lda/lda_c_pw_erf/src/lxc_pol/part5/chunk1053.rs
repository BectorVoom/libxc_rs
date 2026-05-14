//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1053<F: Float>(t2171: F, t6282: F, t6428: F, t2031: F, t6988: F, t1987: F, t1992: F, t3863: F, t571: F, t7815: F, t4763: F, t6239: F, t1318: F, t1466: F, t16907: F, t811: F) -> (F, F, F, F, F, F, F, F) {
    let t21915 = 4.0 / 15.0 * t2171 * t6282;
    let t21917 = 8.0 / 15.0 * t2171 * t6428;
    let t21919 = 8.0 / 15.0 * t6988 * t2031;
    let t21921 = 16.0 / 15.0 * t6988 * t1987;
    let t21923 = 8.0 / 9.0 * t6988 * t1992;
    let t21925 = t571 * t3863 * t7815;
    let t21926 = 8.0 / 45.0 * t21925;
    let t21927 = t4763 * t6239;
    let t21928 = 32.0 / 45.0 * t21927;
    let t21932 = 4.0 / 5.0 * t1318 * t1466 * t16907 * t811;
    (t21915, t21917, t21919, t21921, t21923, t21926, t21928, t21932)
}
