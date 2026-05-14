//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1029/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1029<F: Float>(t20708: F, t2170: F, t3138: F, t6177: F, t20306: F, t20669: F, t20670: F, t20676: F, t20682: F, t20687: F, t20691: F, t20700: F, t20702: F, t20703: F, t2084: F, t2253: F, t2255: F, t2277: F, t2278: F, t2312: F, t3223: F, t3257: F, t6195: F) -> (F, F) {
    let t20712 = t3138 * t2170 * t6177 * t20708 / 4.0;
    let t20713 = t20669 + t2312 * t2255 * t2278 * t20670 / 64.0 - 7.0 / 32.0 * t20676 - t2253 * t20306 * t3223 / 192.0 - t2253 * t3257 * t2084 * t20682 / 64.0 + 7.0 / 48.0 * t20687 - t20691 - t20700 + t20702 - t2277 * t3257 * t6195 * t20703 / 192.0 - t20712;
    (t20712, t20713)
}
