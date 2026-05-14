//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1392/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1392<F: Float>(t117204: F, t117206: F, t117207: F, t117211: F, t122021: F, t122024: F, t122029: F, t122036: F, t122041: F, t122044: F, t122047: F, t122050: F, t2785: F, t34078: F, t34122: F, t34125: F, t34218: F, t34232: F, t7278: F) -> (F,) {
    let t122052 = -0.33163888888888888888e-2 * t122021 + t117204 + 0.88437037037037037034e-2 * t122024 + 0.20833333333333333334e-1 * t34122 * t34218 + 0.29479012345679012345e-2 * t122029 + t117206 - 0.22109259259259259259e-2 * t117207 - 0.20833333333333333334e-1 * t7278 * t34232 * t2785 - t117211 - 0.58958024691358024689e-2 * t122036 + 0.11111111111111111112e0 * t34125 * t34078 - 0.24872916666666666666e-2 * t122041 - 0.88437037037037037034e-2 * t122044 + 0.16581944444444444444e-2 * t122047 + 0.16581944444444444444e-2 * t122050;
    (t122052,)
}
