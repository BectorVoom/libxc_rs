//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 583/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk583(t3554: f64, t3558: f64, t4274: f64, t90: f64, t115: f64, t4000: f64, t4004: f64, t409: f64, t95: f64, t3193: f64, t1098: f64, t120: f64, t132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4342 = 0.01918668486830976_f64 * t3554;
    let t4343 = 0.1663958259747173_f64 * t3558;
    let t4346 = t90 * t4274;
    let t4348 = t115 * t4274;
    let t4353 = 0.037002892246025966_f64 * t4000;
    let t4354 = 0.29951248675449116_f64 * t4004;
    let t4360 = t409 * t95;
    let t4361 = t4360 * t3193;
    let t4362 = t1098 * t4361;
    let t4364 = t120 * t132;
    (t4342, t4343, t4346, t4348, t4353, t4354, t4360, t4361, t4362, t4364)
}
