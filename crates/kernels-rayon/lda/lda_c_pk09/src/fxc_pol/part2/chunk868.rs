//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 868/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk868(t2354: f64, t4104: f64, t1076: f64, t2210: f64, t2214: f64, t3121: f64, t3123: f64, t4113: f64, t4134: f64, t4138: f64, t4144: f64, t4146: f64, t4147: f64, t4149: f64, t7706: f64, t7768: f64, t7776: f64, t7962: f64, t98: f64) -> f64 {
    let t9015 = t2354 * t4104;
    let t9018 = t4134 + 0.09983749558483038_f64 * t3121 + 0.09983749558483038_f64 * t3123 + t4138 + t4113 * t2210 / 6.0_f64 + t4144 + t1076 * t7962 / 6.0_f64 - t4146 + t1076 * t7768 / 6.0_f64 + t4147 / 9.0_f64 + t4149 / 9.0_f64 + t4113 * t2214 / 6.0_f64 + t1076 * t7776 / 6.0_f64 + t1076 * t7706 / 6.0_f64 - t9015 * t98 / 6.0_f64;
    t9018
}
