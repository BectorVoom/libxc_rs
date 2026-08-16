//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1162/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1162<F: Float>(t15035: F, t859: F, t14026: F, t14481: F, t14483: F, t14485: F, t14487: F, t14489: F, t14491: F, t14493: F, t14495: F, t14499: F, t14502: F) -> (F, F) {
    let t15036 = t859 * t15035;
    let t15049 = -t14481 / F::cast_from(192.0_f64) + t14483 / F::cast_from(48.0_f64) - t14485 / F::cast_from(384.0_f64) + t14487 / F::cast_from(96.0_f64) - t14489 / F::cast_from(384.0_f64) + t14491 / F::cast_from(48.0_f64) - t14493 / F::cast_from(192.0_f64) - t14495 / F::cast_from(48.0_f64) + t14499 / F::cast_from(128.0_f64) - t14026 + t14502 / F::cast_from(384.0_f64);
    (t15036, t15049)
}
