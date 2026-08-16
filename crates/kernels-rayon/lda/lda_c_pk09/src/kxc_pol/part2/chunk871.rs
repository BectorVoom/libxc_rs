//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 871/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk871(t1098: f64, t9049: f64, t1063: f64, t4368: f64, t7589: f64, t120: f64, t902: f64, t7577: f64, t1076: f64, t1095: f64, t1101: f64, t2355: f64, t3138: f64, t3195: f64, t3201: f64, t4275: f64, t4278: f64, t8892: f64, t8895: f64, t9037: f64, t9040: f64, t9043: f64, t9046: f64, t98: f64) -> (f64, f64, f64) {
    let t9050 = t1098 * t9049;
    let t9054 = t1063 * t9049;
    let t9056 = t4368 * t7589;
    let t9059 = t120 * t902;
    let t9060 = t9059 * t7577;
    let t9069 = 0.14975624337724558_f64 * t3195 + 0.14975624337724558_f64 * t3201 - t9037 * t98 / 6.0_f64 + t9040 / 6.0_f64 + t1101 * t9043 / 3.0_f64 + t1101 * t9046 / 6.0_f64 + t9050 / 6.0_f64 + t1095 * t8892 / 6.0_f64 - t9054 / 6.0_f64 - t1076 * t9056 / 6.0_f64 - t1076 * t9060 / 3.0_f64 + t1095 * t8895 / 6.0_f64 + t2355 * t3138 / 6.0_f64 + t4275 / 9.0_f64 - t4278 / 54.0_f64;
    (t9056, t9060, t9069)
}
