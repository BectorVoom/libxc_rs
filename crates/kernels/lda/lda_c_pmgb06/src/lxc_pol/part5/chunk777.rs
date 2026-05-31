//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 777/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk777<F: Float>(t3615: F, t7277: F, t370: F, t38: F, t2377: F, t760: F) -> (F, F, F, F) {
    let t7278 = t3615 * t7277;
    let t7281 = t370 * t7277;
    let t7283 = F::cast_from(17.53815_f64) * t38 * t7281;
    let t7284 = t2377 * t760;
    (t7278, t7281, t7283, t7284)
}
