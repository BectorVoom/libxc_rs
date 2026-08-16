//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1148/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1148<F: Float>(t13440: F, t4620: F, t519: F, t4900: F, t581: F, t4842: F, t571: F, t13415: F, t13416: F, t13417: F, t13420: F, t13423: F, t13425: F, t13427: F, t13429: F, t13431: F, t13435: F, t13438: F) -> (F, F, F) {
    let t13442 = t519 * t13440 * t4620;
    let t13443 = F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t13442;
    let t13444 = t4900 * t581;
    let t13446 = t571 * t13444 * t4842;
    let t13447 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t13446;
    let t13448 = -t13415 + t13416 + t13417 - t13420 - t13423 + t13425 - t13427 - t13429 + t13431 + t13435 + t13438 - t13443 - t13447;
    (t13443, t13447, t13448)
}
