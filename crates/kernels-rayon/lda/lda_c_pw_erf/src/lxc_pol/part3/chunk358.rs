//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 358/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk358(t1294: f64, t211: f64, t172: f64, t509: f64, t184: f64) -> (f64, f64, f64, f64) {
    let t1295 = t211 * t1294;
    let t1296 = 8.0_f64 / 45.0_f64 * t1295;
    let t1297 = t172 * t509;
    let t1298 = t1297 * t184;
    (t1295, t1296, t1297, t1298)
}
