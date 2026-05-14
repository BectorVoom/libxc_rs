//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 965/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk965<F: Float>(t10954: F, t10962: F, t10966: F, t11062: F, t11070: F, t11556: F, t11559: F, t11563: F, t11566: F, t11574: F, t6327: F, t6519: F, t6527: F, t6642: F, t6649: F, t6650: F, t6655: F) -> (F,) {
    let t12113 = 4.0 * t6527 - 4.0 * t6519 - 2.0 * t11556 + 2.0 * t11559 - 0.168588613077993 * t10962 + 1.3333333333333333 * t11563 - 1.3333333333333333 * t11566 - 0.505765839233979 * t11070 - 0.505765839233979 * t10954 - 0.505765839233979 * t10966 - 0.505765839233979 * t11062 + 2.0 * t11574 + 0.505765839233979 * t6327 + t6642 + t6649 - t6650 - t6655;
    (t12113,)
}
