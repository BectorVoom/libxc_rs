//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1272/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1272<F: Float>(t1: F, t3921: F, t5470: F, t12461: F, t12463: F, t12465: F, t12474: F, t12480: F, t12482: F, t12485: F, t12488: F, t12491: F, t12495: F, t12497: F, t12499: F) -> F {
    let t15015 = t5470 * t1 * t3921;
    let t15017 = -t12461 - t12463 + t12465 - t12474 - t12480 - t12482 - t12485 - t12488 - t12491 - t12495 + F::new(0.001515438175925926) * t15015 - t12497 + t12499;
    t15017
}
