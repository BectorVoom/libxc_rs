//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1026/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1026<F: Float>(t19252: F, t19254: F, t19256: F, t19258: F, t19260: F, t19263: F, t19265: F, t19268: F, t19271: F, t19274: F, t19276: F, t15481: F) -> (F, F) {
    let t19277 = t19252 + t19254 + t19256 + t19258 + t19260 + t19263 + t19265 + t19268 + t19271 - t19274 + t19276;
    let t19278 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t15481;
    (t19277, t19278)
}
