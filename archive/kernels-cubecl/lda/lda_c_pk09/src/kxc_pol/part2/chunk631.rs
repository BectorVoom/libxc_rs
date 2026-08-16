//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 631/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk631<F: Float>(t5212: F, t5068: F, t1427: F, t1435: F, t395: F, t5031: F, t1406: F, t5081: F, t353: F, t371: F, t1468: F, t381: F) -> (F, F, F, F, F, F, F, F) {
    let t5213 = F::cast_from(2.0_f64) * t5212;
    let t5215 = F::cast_from(0.112392408718662_f64) * t5068;
    let t5225 = t1427 * t1435;
    let t5227 = t395 * t5031;
    let t5235 = t1406 * t5081;
    let t5238 = t353 * t353;
    let t5239 = F::cast_from(1.0_f64) / t5238;
    let t5247 = t371 * t371;
    let t5248 = F::cast_from(1.0_f64) / t5247;
    let t5253 = t381 * t1468;
    (t5213, t5215, t5225, t5227, t5235, t5239, t5248, t5253)
}
