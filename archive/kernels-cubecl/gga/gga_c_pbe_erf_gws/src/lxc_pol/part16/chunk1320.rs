//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1320/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1320<F: Float>(t27047: F, t4216: F, t9296: F, t938: F, t52154: F, t52294: F, t52393: F, t52534: F, t53426: F, t53432: F, t53435: F, t53439: F, t53444: F, t53449: F, t53460: F, t53468: F, t53476: F, t53481: F, t6793: F, t8629: F, t8793: F) -> F {
    let t55182 = t27047 * t9296 * t4216 * t938;
    let t55187 = t53426 / F::cast_from(24.0_f64) - t53432 / F::cast_from(768.0_f64) - t53435 / F::cast_from(192.0_f64) - t53439 / F::cast_from(384.0_f64) - t8793 * t52154 / F::cast_from(12.0_f64) + t53444 / F::cast_from(96.0_f64) + t53449 / F::cast_from(384.0_f64) - t53460 / F::cast_from(768.0_f64) + t8629 * t52534 / F::cast_from(48.0_f64) + t8793 * t52393 / F::cast_from(24.0_f64) - t8629 * t52294 / F::cast_from(24.0_f64) + t53468 / F::cast_from(768.0_f64) - t6793 * t55182 / F::cast_from(8.0_f64) + t53476 / F::cast_from(96.0_f64) + t53481 / F::cast_from(192.0_f64);
    t55187
}
