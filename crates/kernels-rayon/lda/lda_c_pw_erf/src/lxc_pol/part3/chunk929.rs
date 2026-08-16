//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 929/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk929(t1361: f64, t925: f64, t9812: f64, t155: f64, t188: f64, t1325: f64, t1442: f64, t2176: f64, t524: f64, t519: f64, t1612: f64, t610: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10115 = t925 * t1361;
    let t10145 = 0.01959135802469136_f64 * t9812;
    let t10162 = t155 * t188;
    let t10164 = t1325 * t10162 * t1442;
    let t10166 = t2176 * t524;
    let t10167 = t519 * t10166;
    let t10169 = t1612 * t610;
    (t10115, t10145, t10162, t10164, t10166, t10167, t10169)
}
