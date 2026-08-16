//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 955/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk955(t10048: f64, t10050: f64, t10052: f64, t10060: f64, t10062: f64, t1451: f64, t2607: f64, t5632: f64, t5714: f64, t9885: f64, t9887: f64, t9890: f64, t9892: f64) -> f64 {
    let t10067 = -t10048 / 18.0_f64 - t10050 / 18.0_f64 - t10052 / 18.0_f64 - 0.14975624337724558_f64 * t9885 - 0.14975624337724558_f64 * t9887 + 0.037002892246025966_f64 * t9890 + 0.037002892246025966_f64 * t9892 - t10060 / 18.0_f64 - t10062 * t1451 / 6.0_f64 - t2607 * t5632 / 6.0_f64 + t5714;
    t10067
}
