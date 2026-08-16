//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1295/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1295(t13232: f64, t13234: f64, t13239: f64, t13242: f64, t13245: f64, t13248: f64, t13251: f64, t13253: f64, t13256: f64, t13259: f64, t13262: f64, t13264: f64, t13269: f64, t13274: f64) -> f64 {
    let t15075 = t13232 - t13234 - t13239 - t13242 + t13245 + t13248 + t13251 - t13253 - t13256 - t13259 - t13262 + t13264 + t13269 + t13274;
    t15075
}
