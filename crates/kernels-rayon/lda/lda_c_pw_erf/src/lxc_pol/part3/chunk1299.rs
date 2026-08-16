//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1299/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1299(t13403: f64, t13405: f64, t13407: f64, t13408: f64, t13409: f64, t13410: f64, t13411: f64, t13412: f64, t13413: f64, t13415: f64, t13416: f64, t13417: f64, t13420: f64, t13423: f64) -> f64 {
    let t15083 = t13403 - t13405 + t13407 - t13408 - t13409 - t13410 - t13411 + t13412 + t13413 - t13415 + t13416 + t13417 - t13420 - t13423;
    t15083
}
