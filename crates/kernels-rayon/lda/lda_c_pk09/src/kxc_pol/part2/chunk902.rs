//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 902/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk902(t44: f64, t2: f64, t2140: f64, t258: f64, t620: f64, t2605: f64, t5647: f64, t332: f64, t299: f64, t94: f64, t1299: f64, t333: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t9564 = t44 * t2;
    let t9568 = piecewise3(t45, 0.0_f64, 2.0_f64 * t2140 * t620 + 4.0_f64 * t258 * t9564);
    let t9578 = t2605 * t5647;
    let t9579 = t9578 * t332;
    let t9580 = t94 * t299;
    let t9581 = t9580 * t1299;
    let t9582 = t333 * t9581;
    (t9568, t9578, t9579, t9581, t9582)
}
