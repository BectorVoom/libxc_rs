//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 684/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk684<F: Float>(t4035: F, t4047: F, t4169: F, t4172: F, t4174: F, t4176: F, t4178: F, t4180: F) -> (F,) {
    let t4182 = t4169 / 96.0 - t4172 / 96.0 - t4035 - t4174 / 48.0 + t4176 / 768.0 - t4178 / 768.0 - t4047 - t4180 / 384.0;
    (t4182,)
}
