//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 936/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk936<F: Float>(t11598: F, t11602: F, t11604: F, t11606: F, t11613: F, t11615: F, t11620: F, t11625: F, t11632: F, t11635: F, t2277: F, t2312: F, t2343: F, t8901: F, t9415: F, t9425: F) -> (F,) {
    let t11638 = 7.0 / 288.0 * t11598 - t11602 - t9415 - t8901 - t11604 - t2312 * t11606 / 384.0 + t11613 + t2343 * t11615 / 192.0 + t2343 * t11620 / 384.0 + t2277 * t11625 / 768.0 - t11632 - t9425 * t11635 / 128.0;
    (t11638,)
}
