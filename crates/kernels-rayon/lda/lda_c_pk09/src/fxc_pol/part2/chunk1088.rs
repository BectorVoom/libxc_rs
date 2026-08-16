//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1088/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1088(t10954: f64, t10962: f64, t10966: f64, t11062: f64, t11070: f64, t11556: f64, t11559: f64, t11563: f64, t11566: f64, t11574: f64, t6327: f64, t6519: f64, t6527: f64, t6882: f64, t6889: f64, t6890: f64, t6895: f64) -> f64 {
    let t11950 = 0.4537481858318121_f64 * t6527 - 0.4537481858318121_f64 * t6519 - 0.22687409291590604_f64 * t11556 + 0.22687409291590604_f64 * t11559 - 0.015084944665313014_f64 * t10962 + 0.15124939527727072_f64 * t11563 - 0.15124939527727072_f64 * t11566 - 0.04525483399593904_f64 * t11070 - 0.04525483399593904_f64 * t10954 - 0.04525483399593904_f64 * t10966 - 0.04525483399593904_f64 * t11062 + 0.22687409291590604_f64 * t11574 + 0.04525483399593904_f64 * t6327 + t6882 + t6889 - t6890 - t6895;
    t11950
}
