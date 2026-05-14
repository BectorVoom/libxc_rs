//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 862/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk862<F: Float>(t7438: F, t7442: F, t7447: F, t7451: F, t7456: F, t7461: F, t7466: F, t7472: F, t7474: F, t7476: F, t7478: F, t7479: F, t7480: F, t7482: F, t7489: F, t7494: F) -> (F,) {
    let t8430 = t7438 - t7442 - t7447 + t7451 + t7456 - t7461 + t7466 - t7472 - t7474 - t7476 - t7478 + t7479 + t7480 + t7482 - t7489 + t7494;
    (t8430,)
}
