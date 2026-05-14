//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 683/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk683<F: Float>(t1576: F, t4866: F, t848: F, t955: F, t839: F, t947: F, t4182: F, t99: F) -> (F, F, F, F) {
    let t4906 = t1576 * t4866;
    let t4909 = t955 * t848;
    let t4911 = t947 * t839;
    let t4913 = t99 * t4182;
    (t4906, t4909, t4911, t4913)
}
