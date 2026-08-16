//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 583/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk583<F: Float>(t1691: F, t3216: F, t156: F, t1678: F, t426: F, t1664: F, t411: F) -> (F, F, F, F) {
    let t3217 = t1691 * t3216;
    let t3219 = t156 * t1678;
    let t3220 = t426 * t3219;
    let t3222 = t1664 * t411;
    (t3217, t3219, t3220, t3222)
}
