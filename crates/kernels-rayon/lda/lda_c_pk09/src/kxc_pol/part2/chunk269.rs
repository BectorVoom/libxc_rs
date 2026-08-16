//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 269/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk269(t51: f64, t1206: f64, t1207: f64, t1189: f64, t1195: f64, t1204: f64, t278: f64, t1192: f64, t1203: f64, zeta_threshold: f64) -> (f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t1208 = t1206 * t1207;
    let t1211 = t1189 * t1204 + 1.28_f64 * t1195 * t1208;
    let t1212 = t278 * t1211;
    let t1213 = piecewise3(t52, t1192, t1212);
    let t1214 = t1203 + t1213;
    (t1212, t1214)
}
