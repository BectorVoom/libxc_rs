//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1150/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1150<F: Float>(t27047: F, t4216: F, t9296: F, t938: F, t52154: F, t52294: F, t52393: F, t52534: F, t53426: F, t53432: F, t53435: F, t53439: F, t53444: F, t53449: F, t53460: F, t53468: F, t53476: F, t53481: F, t6793: F, t8629: F, t8793: F) -> (F,) {
    let t55182 = t27047 * t9296 * t4216 * t938;
    let t55187 = t53426 / 24.0 - t53432 / 768.0 - t53435 / 192.0 - t53439 / 384.0 - t8793 * t52154 / 12.0 + t53444 / 96.0 + t53449 / 384.0 - t53460 / 768.0 + t8629 * t52534 / 48.0 + t8793 * t52393 / 24.0 - t8629 * t52294 / 24.0 + t53468 / 768.0 - t6793 * t55182 / 8.0 + t53476 / 96.0 + t53481 / 192.0;
    (t55187,)
}
