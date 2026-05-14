//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 372/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk372<F: Float>(t1162: F, t353: F, t338: F, t1115: F, t1120: F, t1146: F, t335: F, t833: F, t842: F, t844: F) -> (F, F, F) {
    let t1163 = t353 * t1162;
    let t1164 = t338 * t1163;
    let t1167 = t1115 * t833 / 96.0 - t842 - t844 * t1120 / 48.0 + t335 * t1146 / 96.0 - t335 * t1164 / 96.0;
    (t1163, t1164, t1167)
}
