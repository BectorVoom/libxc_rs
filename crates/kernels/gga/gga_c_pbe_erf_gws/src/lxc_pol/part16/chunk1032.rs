//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1032/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1032<F: Float>(t14185: F, t3212: F, t9283: F, t4227: F, t938: F, t2409: F, t3067: F, t1161: F, t353: F, t859: F, t14026: F, t14481: F, t14483: F, t14485: F, t14487: F, t14489: F, t14491: F, t14493: F, t14495: F, t14499: F, t14502: F) -> (F, F, F, F, F, F, F) {
    let t15021 = t14185 * t3212;
    let t15022 = t9283 * t15021;
    let t15025 = t4227 * t938;
    let t15027 = t2409 * t3067 * t15025;
    let t15034 = t14185 * t1161;
    let t15035 = t353 * t15034;
    let t15036 = t859 * t15035;
    let t15049 = -t14481 / 192.0 + t14483 / 48.0 - t14485 / 384.0 + t14487 / 96.0 - t14489 / 384.0 + t14491 / 48.0 - t14493 / 192.0 - t14495 / 48.0 + t14499 / 128.0 - t14026 + t14502 / 384.0;
    (t15021, t15022, t15025, t15027, t15034, t15036, t15049)
}
