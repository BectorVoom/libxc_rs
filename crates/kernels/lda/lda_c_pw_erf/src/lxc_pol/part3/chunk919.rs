//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 919/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk919<F: Float>(t1257: F, t925: F, t1247: F, t325: F, t3537: F, t3892: F, t56: F, t3495: F, t3527: F, t1953: F, t506: F, t1253: F) -> (F, F, F, F, F, F, F, F) {
    let t9828 = t925 * t1257;
    let t9832 = t925 * t1247;
    let t9834 = t325 * t3537;
    let t9836 = t56 * t3892;
    let t9840 = t325 * t3495;
    let t9845 = t325 * t3527;
    let t9847 = t1953 * t506;
    let t9866 = t925 * t1253;
    (t9828, t9832, t9834, t9836, t9840, t9845, t9847, t9866)
}
