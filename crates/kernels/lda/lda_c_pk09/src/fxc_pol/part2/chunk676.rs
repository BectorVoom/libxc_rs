//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 676/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk676<F: Float>(t6319: F, t1240: F, t431: F, t10: F, t1731: F) -> (F, F, F) {
    let t6320 = F::new(2.0) * t6319;
    let t6321 = t1240 * t431;
    let t6322 = t6321 * t10;
    let t6323 = t6322 * t1731;
    (t6320, t6322, t6323)
}
