//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 549/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk549<F: Float>(t1109: F, t817: F, t1076: F, t745: F, t2102: F, t2107: F, t2848: F, t3028: F, t323: F, t818: F) -> (F, F, F) {
    let t3030 = t1109 * t817;
    let t3033 = t1076 * t745;
    let t3037 = -t1076 * t2102 + 2.0 * t2107 * t3033 - t2848 * t818 + t3028 * t323 - t3030 * t745;
    (t3030, t3033, t3037)
}
