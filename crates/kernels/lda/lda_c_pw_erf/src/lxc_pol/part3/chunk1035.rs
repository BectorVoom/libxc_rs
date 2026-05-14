//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1035/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1035<F: Float>(t10027: F, t4476: F, t3824: F, t3974: F, t4475: F, t13998: F, t13999: F, t14000: F, t14001: F, t14002: F, t14005: F, t14007: F, t14010: F, t14013: F, t14017: F, t14020: F) -> (F, F, F) {
    let t14022 = 16.0 / 15.0 * t10027 * t4476;
    let t14025 = 8.0 / 15.0 * t3974 * t4475 * t3824;
    let t14026 = -t13998 + t13999 + t14000 + t14001 + t14002 + t14005 - t14007 - t14010 - t14013 - t14017 - t14020 - t14022 - t14025;
    (t14022, t14025, t14026)
}
