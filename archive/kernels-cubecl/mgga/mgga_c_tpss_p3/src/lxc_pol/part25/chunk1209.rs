//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1209/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1209<F: Float>(t19466: F, t19479: F, t19491: F, t18438: F, t18452: F, t18466: F, t18934: F, t18943: F, t19471: F, t19473: F, t19477: F, t19481: F, t19483: F, t19485: F, t19489: F, t19493: F, t19495: F) -> F {
    let t20142 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t19466;
    let t20146 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t19479;
    let t20151 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t19491;
    let t20154 = t18934 + t18438 + t20142 + t19471 / F::cast_from(8.0_f64) - t19473 / F::cast_from(24.0_f64) + t19477 / F::cast_from(384.0_f64) + t20146 + t19481 / F::cast_from(192.0_f64) - t19483 / F::cast_from(768.0_f64) - t19485 / F::cast_from(768.0_f64) + t18452 + t18943 + t18466 + t19489 / F::cast_from(192.0_f64) + t20151 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t19493 - t19495 / F::cast_from(192.0_f64);
    t20154
}
