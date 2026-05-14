//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 596/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk596<F: Float>(t1968: F, t325: F, t1973: F, t2869: F, t4: F) -> (F, F, F, F) {
    let t4602 = t325 * t1968;
    let t4604 = t325 * t1973;
    let t4605 = 0.002518888888888889 * t4604;
    let t4606 = t4 * t2869;
    (t4602, t4604, t4605, t4606)
}
