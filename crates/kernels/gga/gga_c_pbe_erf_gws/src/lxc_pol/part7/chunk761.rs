//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 761/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk761<F: Float>(t6330: F, t6334: F, t6338: F, t6344: F, t6375: F, t6413: F, t6415: F, t6444: F, t6446: F, t6448: F, t6461: F, t6482: F, t6486: F, t6490: F, t6495: F, t6511: F, t6522: F, t6528: F, t6532: F, t6537: F, t6540: F, t6544: F, t6565: F, t6572: F, t6597: F, t6604: F, t6607: F, t6614: F) -> (F, F) {
    let t6733 = t6330 + t6334 - t6338 + t6344 - t6375 - t6413 - t6415 - t6444 + t6446 + t6448 + t6461 - t6482 - t6486 + t6490;
    let t6735 = t6495 - t6511 - t6522 - t6528 + t6532 + t6537 - t6540 - t6544 + t6565 + t6572 - t6597 - t6604 + t6607 + t6614;
    (t6733, t6735)
}
