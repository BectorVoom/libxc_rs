//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 815/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk815<F: Float>(t6495: F, t6511: F, t6522: F, t6528: F, t6532: F, t6537: F, t6540: F, t6544: F, t6565: F, t6572: F, t6597: F, t6604: F, t6607: F, t6614: F) -> F {
    let t6735 = t6495 - t6511 - t6522 - t6528 + t6532 + t6537 - t6540 - t6544 + t6565 + t6572 - t6597 - t6604 + t6607 + t6614;
    t6735
}
