//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 770/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk770<F: Float>(t1092: F, t9036: F, t1063: F, t8092: F, t3248: F, t95: F, t7597: F, t4368: F, t7601: F, t4364: F, t7607: F, t1098: F, t7589: F, t120: F, t902: F, t7577: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9037 = t1092 * t9036;
    let t9040 = t1063 * t8092;
    let t9042 = t3248 * t95;
    let t9043 = t9042 * t7597;
    let t9046 = t4368 * t7601;
    let t9049 = t4364 * t7607;
    let t9050 = t1098 * t9049;
    let t9054 = t1063 * t9049;
    let t9056 = t4368 * t7589;
    let t9059 = t120 * t902;
    let t9060 = t9059 * t7577;
    (t9037, t9040, t9043, t9046, t9049, t9050, t9054, t9056, t9060)
}
