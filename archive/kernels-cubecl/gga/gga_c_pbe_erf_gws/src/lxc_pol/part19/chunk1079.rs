//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1079/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1079<F: Float>(t11416: F, t11418: F, t11421: F, t11448: F, t11458: F, t11463: F, t11472: F, t11477: F, t11482: F, t8823: F, t8826: F, t8835: F) -> F {
    let t12149 = t11416 + t11418 + t11421 + t11448 + t8823 + t8826 + t8835 - t11458 + t11463 - t11472 + t11477 + t11482;
    t12149
}
