//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1087/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1087<F: Float>(t12149: F, t12150: F, t12152: F, t12153: F, t12156: F, t12157: F, t12159: F, t12160: F, t339: F, t338: F, t376: F, t9807: F) -> (F, F, F) {
    let t12163 = t12149 + t12150 + t12152 + t12153 + t12156 + t12157 + t12159 + t12160;
    let t12164 = t339 * t12163;
    let t12166 = t338 * t12164 * t376;
    let t12169 = t376 * t9807;
    (t12164, t12166, t12169)
}
