//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 950/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk950<F: Float>(t1597: F, t2881: F, t2916: F, t1: F, t2872: F, t482: F, t485: F, t1128: F, t19: F, t1098: F, t2830: F, t2833: F) -> (F, F, F, F, F, F, F) {
    let t10778 = t2881 * t2916 * t1597;
    let t10780 = t2872 * t1;
    let t10783 = F::cast_from(0.007901556131563792_f64) * t482 * t10780 * t485;
    let t10784 = t1128 * t19;
    let t10787 = F::cast_from(0.002972565416694299_f64) * t1098 * t10784 * t1597;
    let t10788 = t2830 * t485;
    let t10791 = F::cast_from(0.10359818039161417_f64) * t2833 * t485;
    (t10778, t10780, t10783, t10784, t10787, t10788, t10791)
}
