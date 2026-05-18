//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1257/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1257<F: Float>(t13772: F, t13930: F, t13939: F, t22379: F, t4385: F, t50977: F, t53432: F, t53435: F, t53439: F, t53444: F, t53449: F, t53460: F, t53464: F, t53468: F, t53472: F, t53476: F, t53481: F, t8629: F, t8654: F) -> F {
    let t53483 = -t53432 / F::new(1536.0) - t53435 / F::new(384.0) - t53439 / F::new(768.0) + t53444 / F::new(192.0) + t53449 / F::new(768.0) - t8629 * t50977 / F::new(24.0) - t8654 * t13939 / F::new(48.0) - t8654 * t13772 / F::new(48.0) + t22379 * t13930 / F::new(24.0) - t53460 / F::new(1536.0) + t4385 * t53464 / F::new(96.0) + t53468 / F::new(1536.0) - t4385 * t53472 / F::new(48.0) + t53476 / F::new(192.0) + t53481 / F::new(384.0);
    t53483
}
