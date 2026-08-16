//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 249/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk249(t666: f64, t670: f64, t612: f64, t616: f64, t626: f64, t636: f64, t653: f64, t676: f64, t681: f64, t687: f64) -> (f64, f64, f64, f64, f64) {
    let t1081 = 0.6806222787477182_f64 * t666;
    let t1082 = 0.4537481858318121_f64 * t670;
    let t1086 = 0.04525483399593904_f64 * t612;
    let t1087 = 0.03016988933062603_f64 * t616;
    let t1091 = t1081 + t1082 + 0.6806222787477182_f64 * t676 + 0.6806222787477182_f64 * t681 - 0.6806222787477182_f64 * t687 + t1086 + t1087 + 0.04525483399593904_f64 * t626 + 0.04525483399593904_f64 * t636 - 0.04525483399593904_f64 * t653;
    (t1081, t1082, t1086, t1087, t1091)
}
