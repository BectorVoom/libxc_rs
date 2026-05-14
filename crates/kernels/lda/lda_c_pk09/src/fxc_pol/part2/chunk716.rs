//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 716/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk716<F: Float>(t2237: F, t4050: F, t119: F, t4064: F, t7731: F, t2318: F, t609: F, t96: F, t839: F, t2213: F, t572: F) -> (F, F, F, F) {
    let t7973 = t2237 * t4050;
    let t7974 = t7973 * t119;
    let t7981 = t4064 * t7731;
    let t7988 = t96 * t2318 * t609;
    let t7989 = t839 * t7988;
    let t7991 = t572 * t2213;
    (t7974, t7981, t7989, t7991)
}
