//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1148/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1148(t13440: f64, t4620: f64, t519: f64, t4900: f64, t581: f64, t4842: f64, t571: f64, t13415: f64, t13416: f64, t13417: f64, t13420: f64, t13423: f64, t13425: f64, t13427: f64, t13429: f64, t13431: f64, t13435: f64, t13438: f64) -> (f64, f64, f64) {
    let t13442 = t519 * t13440 * t4620;
    let t13443 = 40.0_f64 / 27.0_f64 * t13442;
    let t13444 = t4900 * t581;
    let t13446 = t571 * t13444 * t4842;
    let t13447 = 8.0_f64 / 9.0_f64 * t13446;
    let t13448 = -t13415 + t13416 + t13417 - t13420 - t13423 + t13425 - t13427 - t13429 + t13431 + t13435 + t13438 - t13443 - t13447;
    (t13443, t13447, t13448)
}
