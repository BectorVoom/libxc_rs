//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 393/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk393<F: Float>(t1800: F, t1927: F, t337: F, t506: F, t1747: F) -> (F, F, F) {
    let t1929 = F::cast_from(18.635258017632964_f64) * t1927 * t1800;
    let t1930 = t506 * t337;
    let t1931 = t1930 * t1747;
    (t1929, t1930, t1931)
}
