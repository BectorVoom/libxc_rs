//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1198/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1198<F: Float>(t14066: F, t14070: F, t14072: F, t14074: F, t14076: F, t14078: F, t14083: F, t14088: F, t14090: F, t14093: F, t14096: F, t14099: F, t14100: F) -> F {
    let t14102 = t14066 + t14070 - t14072 + t14074 + t14076 - t14078 - t14083 - t14088 + t14090 + F::new(0.3246312408709453) * t14093 + t14096 + t14099 + F::new(0.3246312408709453) * t14100;
    t14102
}
