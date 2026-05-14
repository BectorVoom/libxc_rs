//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 927/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk927<F: Float>(t462: F, t476: F, t1672: F, t2856: F, t11218: F, t6511: F, t6524: F) -> (F, F, F) {
    let t11517 = t462 * t476;
    let t11520 = t2856 * t1672;
    let t11529 = t6524 * t6511 * t11218;
    (t11517, t11520, t11529)
}
