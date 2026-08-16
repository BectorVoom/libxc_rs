//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 631/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk631(t5212: f64, t5068: f64, t1427: f64, t1435: f64, t395: f64, t5031: f64, t1406: f64, t5081: f64, t353: f64, t371: f64, t1468: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5213 = 2.0_f64 * t5212;
    let t5215 = 0.112392408718662_f64 * t5068;
    let t5225 = t1427 * t1435;
    let t5227 = t395 * t5031;
    let t5235 = t1406 * t5081;
    let t5238 = t353 * t353;
    let t5239 = 1.0_f64 / t5238;
    let t5247 = t371 * t371;
    let t5248 = 1.0_f64 / t5247;
    let t5253 = t381 * t1468;
    (t5213, t5215, t5225, t5227, t5235, t5239, t5248, t5253)
}
