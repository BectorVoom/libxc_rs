//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 799/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk799<F: Float>(t13314: F, t13339: F, t13355: F, t13361: F, t13367: F, t13373: F, t13377: F, t13384: F, t13391: F, t13407: F, t13416: F, t13439: F, t13444: F, t13448: F, t13456: F, t13457: F, t13459: F, t13465: F, t13470: F, t13475: F, t13478: F, t13479: F, t13485: F, t13486: F, t13488: F, t13493: F, t13498: F, t13503: F) -> (F, F) {
    let t13672 = t13314 - t13339 + t13355 - t13361 - t13367 + t13373 + t13377 - t13384 - t13391 - t13407 + t13416 - t13439 + t13444 - t13448;
    let t13674 = t13456 - t13457 - t13459 - t13465 + t13470 - t13475 + t13478 + t13479 + t13485 - t13486 - t13488 - t13493 + t13498 + t13503;
    (t13672, t13674)
}
