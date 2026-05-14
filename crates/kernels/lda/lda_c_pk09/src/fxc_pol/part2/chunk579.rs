//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 579/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk579<F: Float>(t5212: F, t5068: F, t1427: F, t1435: F, t395: F, t5031: F, t1406: F, t5081: F, t353: F, t371: F, t1468: F, t381: F, t1284: F, t5012: F, t364: F, t1319: F, t4998: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5213 = 2.0 * t5212;
    let t5215 = 0.112392408718662 * t5068;
    let t5225 = t1427 * t1435;
    let t5227 = t395 * t5031;
    let t5235 = t1406 * t5081;
    let t5238 = t353 * t353;
    let t5239 = 1.0 / t5238;
    let t5247 = t371 * t371;
    let t5248 = 1.0 / t5247;
    let t5253 = t381 * t1468;
    let t5254 = t5253 * t1284;
    let t5256 = 37.27051603526593 * t5254 * t5012;
    let t5257 = t364 * t1468;
    let t5258 = t5257 * t1284;
    let t5260 = 9.87466743489671 * t5258 * t5012;
    let t5262 = 3.2915558116322368 * t1319 * t4998;
    (t5213, t5215, t5225, t5227, t5235, t5239, t5248, t5256, t5260, t5262)
}
