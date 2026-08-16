//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1133/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1133(t2967: f64, t3604: f64, t3832: f64, t571: f64, t833: f64, t1446: f64, t5426: f64, t13239: f64, t13242: f64, t13245: f64, t13248: f64, t13251: f64, t13253: f64, t13256: f64, t13259: f64, t13262: f64, t13264: f64, t13269: f64) -> (f64, f64, f64) {
    let t13274 = 8.0_f64 / 9.0_f64 * t571 * t3832 * t833 * t3604 * t2967;
    let t13276 = 8.0_f64 / 15.0_f64 * t1446 * t5426;
    let t13277 = -t13239 - t13242 + t13245 + t13248 + t13251 - t13253 - t13256 - t13259 - t13262 + t13264 + t13269 + t13274 + t13276;
    (t13274, t13276, t13277)
}
