//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 665/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk665(t1304: f64, t1435: f64, t1283: f64, t5031: f64, t1287: f64, t1285: f64, t4979: f64, t355: f64, t391: f64, t4990: f64, t1387: f64, t4995: f64) -> (f64, f64, f64, f64, f64) {
    let t6109 = t1304 * t1435;
    let t6116 = t1283 * t5031;
    let t6117 = t6116 * t1287;
    let t6120 = 19.489173774580152_f64 * t1285 * t4979;
    let t6121 = t355 * t5031;
    let t6126 = t391 * t4990;
    let t6127 = t6126 * t1387;
    let t6129 = 21.324527244551554_f64 * t6127 * t4995;
    (t6109, t6117, t6120, t6121, t6129)
}
