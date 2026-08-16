//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 808/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk808<F: Float>(t8092: F, t810: F, t2152: F, t633: F, t849: F, t164: F, t2254: F, t694: F, t896: F, t609: F, t3767: F, t161: F, t7991: F) -> (F, F, F, F, F, F) {
    let t8117 = t810 * t8092;
    let t8119 = t2152 * t633;
    let t8120 = t849 * t8119;
    let t8121 = t164 * t8120;
    let t8124 = t896 * t2254 * t694;
    let t8128 = t896 * t2254 * t609;
    let t8129 = t3767 * t8128;
    let t8131 = t161 * t7991;
    (t8117, t8121, t8124, t8128, t8129, t8131)
}
