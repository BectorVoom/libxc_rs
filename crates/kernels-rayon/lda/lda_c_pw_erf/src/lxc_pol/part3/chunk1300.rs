//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1300/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1300(t11153: f64, t11156: f64, t13425: f64, t13427: f64, t13429: f64, t13431: f64, t13435: f64, t13438: f64, t13443: f64, t13447: f64, t13453: f64, t13458: f64, t13463: f64) -> f64 {
    let t15086 = t13425 - t13427 - t13429 + t13431 + t13435 + t13438 - t13443 - t13447 - t13453 + t13458 + t13463 - t11153 - t11156;
    t15086
}
