//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1394/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1394<F: Float>(t2408: F, t3060: F, t55151: F, t55781: F, t57441: F, t57449: F, t57454: F, t57458: F, t57462: F, t57468: F, t57472: F, t57474: F, t57476: F, t57480: F, t57482: F, t57484: F, t57486: F, t9283: F) -> F {
    let t58818 = -t57441 / F::cast_from(768.0_f64) + t57449 / F::cast_from(48.0_f64) - t2408 * t9283 * t55151 * t3060 / F::cast_from(12.0_f64) - t57454 / F::cast_from(768.0_f64) + t57458 / F::cast_from(48.0_f64) + t57462 / F::cast_from(1536.0_f64) + t57468 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t57472 - t55781 - t57474 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t57476 - t57480 / F::cast_from(48.0_f64) - t57482 / F::cast_from(24.0_f64) - t57484 / F::cast_from(12.0_f64) - t57486 / F::cast_from(12.0_f64);
    t58818
}
