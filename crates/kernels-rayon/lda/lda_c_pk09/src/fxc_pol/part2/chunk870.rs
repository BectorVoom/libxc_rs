//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 870/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk870(t10: f64, t2392: f64, t88: f64, t1092: f64, t1063: f64, t8092: f64, t3248: f64, t95: f64, t7597: f64, t4368: f64, t7601: f64, t4364: f64, t7607: f64) -> (f64, f64, f64, f64, f64) {
    let t9036 = t2392 * t88 * t10;
    let t9037 = t1092 * t9036;
    let t9040 = t1063 * t8092;
    let t9042 = t3248 * t95;
    let t9043 = t9042 * t7597;
    let t9046 = t4368 * t7601;
    let t9049 = t4364 * t7607;
    (t9037, t9040, t9043, t9046, t9049)
}
