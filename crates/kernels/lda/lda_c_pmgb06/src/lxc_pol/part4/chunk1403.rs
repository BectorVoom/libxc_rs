//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1403/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1403<F: Float>(t16313: F, t16315: F, t16321: F, t16336: F, t16338: F, t16339: F, t16340: F, t16342: F, t16345: F, t16347: F, t16349: F, t16351: F, t16353: F, t16357: F, t16362: F) -> F {
    let t18231 = -t16313 - t16315 - t16321 + t16336 + t16338 - t16339 - t16340 + t16342 + t16345 + t16347 + t16349 + t16351 + t16353 - t16357 - t16362;
    t18231
}
