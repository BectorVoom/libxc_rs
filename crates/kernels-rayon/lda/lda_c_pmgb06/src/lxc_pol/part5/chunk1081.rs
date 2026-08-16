//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1081/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1081(t19989: f64, t5083: f64, t5084: f64, t10693: f64, t10696: f64, t12461: f64, t12463: f64, t19985: f64, t19986: f64, t19987: f64, t19988: f64, t19992: f64, t19995: f64, t19998: f64) -> (f64, f64) {
    let t20001 = t5083 * t5084 * t19989 / 9.0_f64;
    let t20003 = -t19985 - t12461 - t12463 + t19986 + t19987 + t19988 + t19992 + t19995 - t19998 - t20001 + 0.0011033703703703704_f64 * t10693 + t10696;
    (t20001, t20003)
}
