//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 962/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk962<F: Float>(t8407: F, t8416: F, t8418: F, t8421: F, t8427: F, t8428: F, t8430: F, t8432: F, t8442: F, t8443: F, t8446: F, t8449: F, t8452: F, t8453: F, t8455: F, t8456: F) -> F {
    let t8460 = t8407 + t8416 + t8418 + t8421 + t8427 + t8428 + t8430 + t8432 + t8442 + t8443 + t8446 + t8449 + t8452 + t8453 + t8455 + t8456;
    t8460
}
