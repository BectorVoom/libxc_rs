//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 538/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk538<F: Float>(t1039: F, t133: F, t131: F, t3554: F, t3558: F, t4274: F, t90: F, t115: F, t4000: F, t4004: F, t409: F, t95: F, t3193: F, t1098: F, t120: F, t132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4334 = t133 * t1039;
    let t4335 = t131 * t4334;
    let t4342 = 0.01918668486830976 * t3554;
    let t4343 = 0.1663958259747173 * t3558;
    let t4346 = t90 * t4274;
    let t4348 = t115 * t4274;
    let t4353 = 0.037002892246025966 * t4000;
    let t4354 = 0.29951248675449116 * t4004;
    let t4360 = t409 * t95;
    let t4361 = t4360 * t3193;
    let t4362 = t1098 * t4361;
    let t4364 = t120 * t132;
    (t4335, t4342, t4343, t4346, t4348, t4353, t4354, t4360, t4361, t4362, t4364)
}
