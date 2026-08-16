//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2259/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2259<F: Float>(t104450: F, t105709: F, t118: F, t13537: F, t29432: F, t4293: F, t7586: F, t98455: F, t98458: F, t98461: F, t98463: F, t98467: F, t98472: F, t98474: F, t98477: F, t98483: F, t98486: F, t98489: F, t98491: F, t98494: F, t98499: F, t98501: F, t98522: F) -> F {
    let t105712 = -t98455 - t98458 + t98461 - t98463 - t98467 - t98472 - t98474 - t98477 - t98483 - t98486 - t98489 - t98491 - t98494 - t98499 + t98501 - F::cast_from(4.0_f64) * t29432 * t4293 - F::cast_from(2.0_f64) * t7586 * t13537 - t118 * (t104450 + t105709) - t98522;
    t105712
}
