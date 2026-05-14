//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 608/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk608<F: Float>(t1328: F, t3859: F, t1325: F, t1475: F, t581: F) -> (F, F, F, F) {
    let t3860 = t3859 * t1328;
    let t3861 = t1325 * t3860;
    let t3862 = 32.0 / 45.0 * t3861;
    let t3863 = t1475 * t581;
    (t3860, t3861, t3862, t3863)
}
