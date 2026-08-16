//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 334/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk334(t1579: f64, t314: f64, t306: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64, t323: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1580 = t314 * t1579;
    let t1581 = t1580 * t306;
    let t1584 = 1.5323028051206833_f64 * t1243;
    let t1586 = 0.5107676017068944_f64 * t1255;
    let t1588 = 0.3056501876701794_f64 * t1263;
    let t1590 = 0.1018833958900598_f64 * t1272;
    let t1592 = t1584 - 1.5323028051206833_f64 * t1251 + t1586 + 1.5323028051206833_f64 * t1259 + t1588 - 0.3056501876701794_f64 * t1268 + t1590 + 0.3056501876701794_f64 * t1275;
    let t1593 = t323 * t1592;
    (t1580, t1581, t1584, t1586, t1588, t1590, t1592, t1593)
}
