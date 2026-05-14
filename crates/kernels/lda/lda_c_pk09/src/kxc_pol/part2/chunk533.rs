//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 533/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk533<F: Float>(t3743: F, t932: F, t113: F, t61: F, t650: F, t891: F, t733: F, t861: F, t62: F) -> (F, F, F, F, F, F) {
    let t4085 = t932 * t3743;
    let t4086 = t61 * t113;
    let t4088 = t891 * t4086 * t650;
    let t4091 = t861 * t733;
    let t4092 = t4091 * t3743;
    let t4093 = t62 * t113;
    (t4085, t4086, t4088, t4091, t4092, t4093)
}
