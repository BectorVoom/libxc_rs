//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1381/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1381<F: Float>(t15261: F, t15263: F, t15268: F, t15270: F, t15273: F, t15278: F, t15280: F, t15282: F, t15285: F, t15287: F, t15290: F, t15293: F, t15295: F, t15298: F, t15302: F) -> F {
    let t18162 = -t15261 + t15263 - t15268 + t15270 + t15273 + t15278 - t15280 - t15282 - t15285 - t15287 - t15290 - t15293 + t15295 + t15298 + t15302;
    t18162
}
