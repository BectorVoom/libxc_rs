//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 477/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk477<F: Float>(t2: F, t258: F, t263: F, t3: F, t142: F, t92: F) -> (F, F, F, F) {
    let t2962 = t2 * t258;
    let t2964 = t3 * t263;
    let t2965 = 6.0 * t2964;
    let t2971 = t142 * t92;
    (t2962, t2964, t2965, t2971)
}
