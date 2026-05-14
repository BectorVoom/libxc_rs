//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1113/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1113<F: Float>(t11153: F, t11156: F, t13425: F, t13427: F, t13429: F, t13431: F, t13435: F, t13438: F, t13443: F, t13447: F, t13453: F, t13458: F, t13463: F, t11159: F, t11160: F, t11162: F, t11164: F, t11166: F, t11168: F, t13465: F, t13466: F, t13467: F, t13469: F, t13471: F, t13475: F, t13477: F) -> (F, F) {
    let t15086 = t13425 - t13427 - t13429 + t13431 + t13435 + t13438 - t13443 - t13447 - t13453 + t13458 + t13463 - t11153 - t11156;
    let t15092 = t11159 + 2.0 / 9.0 * t11160 + 4.0 / 3.0 * t11162 - 2.0 / 9.0 * t11164 - 2.0 / 3.0 * t11166 - 0.040518518518518516 * t11168 - t13465 + t13466 - t13467 + t13469 + t13471 - t13475 - t13477;
    (t15086, t15092)
}
