//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 306/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk306(t372: f64, t623: f64, t1349: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1350 = t372 * t623;
    let t1351 = t1349 * t1350;
    let t1354 = 4.0_f64 * t1243;
    let t1356 = 1.3333333333333333_f64 * t1255;
    let t1358 = 0.821419393556371_f64 * t1263;
    let t1360 = 0.2738064645187903_f64 * t1272;
    let t1362 = t1354 - 4.0_f64 * t1251 + t1356 + 4.0_f64 * t1259 + t1358 - 0.821419393556371_f64 * t1268 + t1360 + 0.821419393556371_f64 * t1275;
    let t1363 = 1.0_f64 / t371;
    (t1350, t1351, t1354, t1356, t1358, t1360, t1362, t1363)
}
