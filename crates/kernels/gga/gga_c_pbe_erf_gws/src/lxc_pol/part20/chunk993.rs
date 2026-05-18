//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 993/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk993<F: Float>(t10450: F, t10454: F, t10458: F, t10461: F, t10464: F, t10468: F, t10474: F, t10476: F, t10478: F, t10480: F, t10484: F, t10487: F, t10491: F, t10495: F, t10497: F, t10499: F) -> F {
    let t11205 = t10450 + t10454 - t10458 - t10461 - t10464 - t10468 + t10474 - t10476 - t10478 - t10480 - t10484 + t10487 - t10491 - t10495 - t10497 + t10499;
    t11205
}
