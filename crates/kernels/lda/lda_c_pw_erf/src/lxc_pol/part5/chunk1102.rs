//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1102/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1102<F: Float>(t14684: F, t14692: F, t14698: F, t1870: F, t20294: F, t20356: F, t20374: F, t20390: F, t20396: F, t20403: F, t20406: F, t20409: F, t20412: F, t20433: F, t20434: F) -> F {
    let t20529 = t20356 - F::new(62.07318) * t1870 * t20294 - t20374 + t20390 + t20396 + t20403 - t20406 + t20409 + t20412 - t14684 - t20433 + t14692 + F::new(6.89702) * t14698 + t20434;
    t20529
}
