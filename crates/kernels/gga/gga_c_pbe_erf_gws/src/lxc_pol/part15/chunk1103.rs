//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1103/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1103<F: Float>(t13772: F, t13930: F, t13939: F, t22379: F, t4385: F, t50977: F, t53432: F, t53435: F, t53439: F, t53444: F, t53449: F, t53460: F, t53464: F, t53468: F, t53472: F, t53476: F, t53481: F, t8629: F, t8654: F) -> (F,) {
    let t53483 = -t53432 / 1536.0 - t53435 / 384.0 - t53439 / 768.0 + t53444 / 192.0 + t53449 / 768.0 - t8629 * t50977 / 24.0 - t8654 * t13939 / 48.0 - t8654 * t13772 / 48.0 + t22379 * t13930 / 24.0 - t53460 / 1536.0 + t4385 * t53464 / 96.0 + t53468 / 1536.0 - t4385 * t53472 / 48.0 + t53476 / 192.0 + t53481 / 384.0;
    (t53483,)
}
