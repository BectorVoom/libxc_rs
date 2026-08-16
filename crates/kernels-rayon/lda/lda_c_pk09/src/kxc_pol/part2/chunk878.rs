//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 878/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk878(t7831: f64, t96: f64, t93: f64, t1067: f64, t2363: f64, t1101: f64, t8141: f64, t1076: f64, t7991: f64, t2355: f64, t1011: f64, t2210: f64, t4342: f64, t4343: f64, t4440: f64, t7768: f64, t7962: f64, t8121: f64, t90: f64, t9043: f64, t9046: f64) -> (f64, f64) {
    let t9158 = t96 * t7831;
    let t9159 = t93 * t9158;
    let t9171 = t2363 * t1067;
    let t9173 = t1101 * t8141;
    let t9175 = t1076 * t7991;
    let t9177 = t2355 * t1067;
    let t9179 = t1076 * t8141;
    let t9181 = -t1076 * t9043 / 3.0_f64 - t1076 * t9046 / 6.0_f64 - t90 * t9159 / 6.0_f64 + t4342 + t4343 - 0.14975624337724558_f64 * t8121 - t2363 * t1011 / 6.0_f64 - t4440 * t2210 / 6.0_f64 - t1101 * t7962 / 6.0_f64 - t1101 * t7768 / 6.0_f64 + t9171 / 9.0_f64 + t9173 / 9.0_f64 - t9175 / 9.0_f64 + t9177 / 9.0_f64 - t9179 / 9.0_f64;
    (t9159, t9181)
}
