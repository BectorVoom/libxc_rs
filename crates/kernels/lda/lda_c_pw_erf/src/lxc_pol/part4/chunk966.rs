//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 966/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk966<F: Float>(t10770: F, t1138: F, t1597: F, t1586: F, t2910: F, t485: F, t2881: F, t2916: F, t1: F, t2872: F, t482: F, t1128: F, t19: F, t1098: F, t2830: F, t2833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10772 = t10770 * t1138 * t1597;
    let t10775 = t1586 * t2910 * t485;
    let t10778 = t2881 * t2916 * t1597;
    let t10780 = t2872 * t1;
    let t10783 = 0.007901556131563792 * t482 * t10780 * t485;
    let t10784 = t1128 * t19;
    let t10787 = 0.002972565416694299 * t1098 * t10784 * t1597;
    let t10788 = t2830 * t485;
    let t10791 = 0.10359818039161417 * t2833 * t485;
    (t10772, t10775, t10778, t10780, t10783, t10784, t10787, t10788, t10791)
}
