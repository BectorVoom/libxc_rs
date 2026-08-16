//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1302/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1302<F: Float>(t13479: F, t13481: F, t13483: F, t13486: F, t13489: F, t13491: F, t13494: F, t13496: F, t13498: F, t13500: F, t13505: F, t13508: F, t13510: F) -> F {
    let t15094 = -t13479 - t13481 - t13483 - t13486 + t13489 - t13491 - t13494 - t13496 + t13498 + t13500 + t13505 + t13508 + t13510;
    t15094
}
