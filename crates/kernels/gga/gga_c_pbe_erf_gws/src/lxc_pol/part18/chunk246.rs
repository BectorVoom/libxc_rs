//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 246/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk246<F: Float>(t153: F, t274: F, t542: F, t386: F, t407: F, t411: F, t416: F, t429: F, t462: F, t464: F, t469: F, t474: F) -> (F, F) {
    let t744 = F::new(0.56945186695483624892e0) * t153 * t542 * t274;
    let t745 = t386 + t407 + t411 - t416 + t429 + t462 + t464 - t469 - t474;
    (t744, t745)
}
