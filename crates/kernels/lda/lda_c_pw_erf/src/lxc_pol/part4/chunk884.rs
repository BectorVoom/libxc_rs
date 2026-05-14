//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 884/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk884<F: Float>(t41: F, t6039: F, t2379: F, t632: F, t153: F, t2357: F, t474: F, t168: F, t2581: F, t635: F, t145: F, t2363: F) -> (F, F, F, F, F) {
    let t7032 = t41 * t6039;
    let t7035 = t2379 * t632;
    let t7038 = t153 * t474 * t2357;
    let t7043 = t168 * t635 * t2581;
    let t7045 = t145 * t2363;
    (t7032, t7035, t7038, t7043, t7045)
}
