//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 940/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk940<F: Float>(t10527: F, t4051: F, t571: F, t1318: F, t3899: F, t4043: F, t1476: F, t3727: F, t3892: F, t9: F, t3895: F, t519: F) -> (F, F, F, F, F) {
    let t10529 = t571 * t10527 * t4051;
    let t10541 = t1318 * t3899 * t4043;
    let t10551 = t3727 * t1476;
    let t10557 = t9 * t3892;
    let t10559 = t519 * t10557 * t3895;
    (t10529, t10541, t10551, t10557, t10559)
}
