//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 366/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk366<F: Float>(t633: F, t902: F, t93: F, t337: F, t544: F, t1747: F) -> (F, F, F, F) {
    let t1848 = t902 * t633;
    let t1849 = t93 * t1848;
    let t1852 = t544 * t337;
    let t1853 = t1852 * t1747;
    (t1848, t1849, t1852, t1853)
}
