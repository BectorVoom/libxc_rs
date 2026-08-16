//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1355/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1355<F: Float>(t23030: F, t23032: F, t23035: F, t23037: F, t23039: F, t23041: F, t23042: F, t23043: F, t23044: F, t23045: F, t23046: F, t23047: F, t23048: F) -> F {
    let t23333 = t23030 + t23032 + t23035 - t23037 + t23039 - t23041 + t23042 + t23043 + t23044 + t23045 - t23046 + t23047 + t23048;
    t23333
}
