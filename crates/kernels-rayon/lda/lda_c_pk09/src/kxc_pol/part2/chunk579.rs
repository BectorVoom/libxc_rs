//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 579/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk579(t3130: f64, t1106: f64, t3290: f64, t1098: f64, t1101: f64, t3230: f64, t3233: f64, t3522: f64, t91: f64, t114: f64, t3163: f64, t1063: f64, t3498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4138 = 0.032891459774245305_f64 * t3130;
    let t4144 = t1106 * t3290 / 6.0_f64;
    let t4146 = t1098 * t3290 / 6.0_f64;
    let t4147 = t1101 * t3230;
    let t4149 = t1101 * t3233;
    let t4165 = t3522 * t91;
    let t4166 = t114 * t4165;
    let t4168 = t4166 * t3163 / 3.0_f64;
    let t4170 = 2.0_f64 / 9.0_f64 * t1063 * t3498;
    (t4138, t4144, t4146, t4147, t4149, t4165, t4168, t4170)
}
