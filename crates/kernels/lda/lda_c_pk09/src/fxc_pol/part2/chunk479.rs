//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 479/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk479<F: Float>(t2972: F, t2974: F, t119: F, t863: F, t1062: F, t721: F, t572: F, t755: F) -> (F, F, F, F) {
    let t2975 = t2972 * t2974;
    let t2977 = t863 * t119;
    let t2980 = t863 * t1062;
    let t2981 = t2980 * t721;
    let t2983 = t572 * t755;
    (t2975, t2977, t2981, t2983)
}
