//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 390/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk390<F: Float>(t1427: F, t645: F, t925: F, t933: F) -> (F, F) {
    let t1429 = F::cast_from(0.12155555555555556_f64) * t645 * t1427;
    let t1432 = -F::cast_from(0.043111111111111114_f64) * t925 + F::cast_from(0.18777777777777777_f64) * t933;
    (t1429, t1432)
}
