//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1021/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1021<F: Float>(t6443: F, t802: F, t11758: F, t14311: F, t14312: F, t14314: F, t14316: F, t19209: F, t19211: F, t19215: F, t19217: F, t19219: F) -> (F, F) {
    let t19221 = t802 * t6443 / F::new(5.0);
    let t19222 = -t19209 + t19211 + t14311 + F::new(0.6492624817418906) * t14312 - F::new(0.2885611029963958) * t14314 - F::new(0.03354522822333102) * t14316 - t19215 - t19217 + t19219 + t19221 + t11758;
    (t19221, t19222)
}
