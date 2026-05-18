//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 491/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk491<F: Float>(t2747: F, t93: F, t481: F, t132: F, t2149: F, t333: F) -> (F, F, F, F) {
    let t2748 = t93 * t2747;
    let t2749 = t481 * t2748;
    let t2751 = t132 * t2149;
    let t2752 = t333 * t2751;
    (t2748, t2749, t2751, t2752)
}
