//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 666/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk666(t1468: f64, t354: f64, t1284: f64, t5012: f64, t323: f64, t359: f64, t5031: f64, t402: f64, t1470: f64, t4943: f64, t1476: f64, t5084: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6130 = t354 * t1468;
    let t6131 = t6130 * t1284;
    let t6133 = 3.7610742193750633_f64 * t6131 * t5012;
    let t6134 = t323 * t1468;
    let t6135 = t6134 * t1284;
    let t6137 = 7.5221484387501265_f64 * t6135 * t5012;
    let t6138 = t359 * t5031;
    let t6149 = t402 * t1468;
    let t6150 = t6149 * t1284;
    let t6152 = 4.855032390388656_f64 * t6150 * t5012;
    let t6154 = 9.477567664245134_f64 * t1470 * t4943;
    let t6155 = t1476 * t5084;
    (t6133, t6137, t6138, t6152, t6154, t6155)
}
