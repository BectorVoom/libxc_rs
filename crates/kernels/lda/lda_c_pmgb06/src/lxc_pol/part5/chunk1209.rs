//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1209/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1209<F: Float>(t19237: F, t19238: F, t19239: F, t19241: F, t19243: F, t19245: F, t19247: F, t19252: F, t19254: F, t19256: F, t19258: F, t19260: F) -> F {
    let t21851 = -t19237 + t19238 - t19239 - t19241 + t19243 + t19245 + t19247 + t19252 + t19254 + t19256 + t19258 + t19260;
    t21851
}
