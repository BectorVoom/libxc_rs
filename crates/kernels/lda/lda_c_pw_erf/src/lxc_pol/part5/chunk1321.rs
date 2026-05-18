//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1321/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1321<F: Float>(t12714: F, t21364: F, t21365: F, t21366: F, t21378: F, t21381: F, t21384: F, t21385: F, t21387: F, t21388: F, t21389: F, t21390: F, t21392: F) -> F {
    let t23242 = t21364 + t21365 + t21366 + F::new(0.0033101111111111113) * t12714 - t21378 - t21381 - t21384 - t21385 + t21387 + t21388 + t21389 + t21390 - t21392;
    t23242
}
