//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1300/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1300<F: Float>(t11153: F, t11156: F, t13425: F, t13427: F, t13429: F, t13431: F, t13435: F, t13438: F, t13443: F, t13447: F, t13453: F, t13458: F, t13463: F) -> F {
    let t15086 = t13425 - t13427 - t13429 + t13431 + t13435 + t13438 - t13443 - t13447 - t13453 + t13458 + t13463 - t11153 - t11156;
    t15086
}
