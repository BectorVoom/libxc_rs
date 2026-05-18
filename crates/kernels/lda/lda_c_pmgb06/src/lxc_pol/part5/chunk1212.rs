//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1212/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1212<F: Float>(t19284: F, t19286: F, t19289: F, t19291: F, t19293: F, t19295: F, t19298: F, t19300: F, t19302: F, t19307: F, t19309: F, t21873: F, t21891: F, t224: F, t44: F) -> F {
    let t21897 = -t19284 - t19286 - t19289 + t19291 + t19293 - t19295 - t19298 + t19300 + t19302 - t19307 + t19309 - (t21873 / F::new(2.0) + t21891 / F::new(2.0)) * t44 * t224 / F::new(15.0);
    t21897
}
