//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1257/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1257<F: Float>(t13772: F, t13930: F, t13939: F, t22379: F, t4385: F, t50977: F, t53432: F, t53435: F, t53439: F, t53444: F, t53449: F, t53460: F, t53464: F, t53468: F, t53472: F, t53476: F, t53481: F, t8629: F, t8654: F) -> F {
    let t53483 = -t53432 / F::cast_from(1536.0_f64) - t53435 / F::cast_from(384.0_f64) - t53439 / F::cast_from(768.0_f64) + t53444 / F::cast_from(192.0_f64) + t53449 / F::cast_from(768.0_f64) - t8629 * t50977 / F::cast_from(24.0_f64) - t8654 * t13939 / F::cast_from(48.0_f64) - t8654 * t13772 / F::cast_from(48.0_f64) + t22379 * t13930 / F::cast_from(24.0_f64) - t53460 / F::cast_from(1536.0_f64) + t4385 * t53464 / F::cast_from(96.0_f64) + t53468 / F::cast_from(1536.0_f64) - t4385 * t53472 / F::cast_from(48.0_f64) + t53476 / F::cast_from(192.0_f64) + t53481 / F::cast_from(384.0_f64);
    t53483
}
