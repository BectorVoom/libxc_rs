//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 761/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk761<F: Float>(t6381: F, t6384: F, t6386: F, t6389: F, t6393: F, t6397: F, t6401: F, t6405: F, t6409: F, t6411: F, t6415: F, t6418: F, t6421: F, t6424: F, t6426: F) -> F {
    let t7188 = t6381 - t6384 - t6386 - t6389 - t6393 + t6397 + t6401 + t6405 + t6409 - t6411 - t6415 - t6418 + t6421 - t6424 - t6426;
    t7188
}
