//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 720/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk720<F: Float>(t2002: F, t4753: F, t1310: F, t2146: F, t2151: F, t219: F) -> (F, F, F) {
    let t4755 = 16.0 / 45.0 * t4753 * t2002;
    let t4757 = 8.0 / 45.0 * t2146 * t1310;
    let t4758 = t2151 * t219;
    (t4755, t4757, t4758)
}
