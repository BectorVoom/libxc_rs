//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1084/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1084<F: Float>(t20366: F, t20370: F, t20377: F, t20381: F, t20385: F, t20392: F, t20395: F, t20400: F, t20410: F, t20414: F, t20424: F, t20428: F, t20431: F, t20435: F, t20437: F, t20453: F, t20459: F, t20468: F, t20489: F, t20493: F, t20499: F, t20511: F) -> (F, F) {
    let t21689 = -t20366 - t20370 + t20377 + t20381 + t20385 + t20392 - t20395 - t20400 + t20410 + t20414 + t20424;
    let t21690 = t20428 - t20431 - t20435 - t20437 - t20453 - t20459 + t20468 - t20489 + t20493 + t20499 + t20511;
    (t21689, t21690)
}
