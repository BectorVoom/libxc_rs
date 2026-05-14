//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 571/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk571<F: Float>(t5049: F, t5153: F, t49: F, t72: F, t1214: F, t1240: F) -> (F, F, F, F) {
    let t5154 = t5049 * t5153;
    let t5155 = t72 * t49;
    let t5156 = t1240 * t1214;
    let t5158 = t5154 * t5155 * t5156;
    (t5154, t5155, t5156, t5158)
}
