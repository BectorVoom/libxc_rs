//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 686/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk686<F: Float>(t1877: F, t6488: F, t1672: F, t1873: F, t431: F, t4993: F, t68: F, t434: F) -> (F, F, F) {
    let t6490 = F::cast_from(12.992782516386768_f64) * t1877 * t6488;
    let t6493 = t1873 * t1672;
    let t6500 = t4993 * t431 * t68;
    let t6501 = t6500 * t434;
    (t6490, t6493, t6501)
}
