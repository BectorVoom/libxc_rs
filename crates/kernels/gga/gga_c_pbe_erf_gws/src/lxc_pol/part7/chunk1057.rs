//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1057<F: Float>(t6373: F, t6484: F, t19993: F, t20264: F, t20527: F, t21146: F, t21148: F, t21155: F, t21158: F, t21159: F, t21161: F, t21174: F, t2255: F, t2277: F, t2278: F, t2300: F, t2343: F, t3235: F, t6350: F, t6598: F, t6637: F, t875: F, t904: F, t929: F) -> (F, F) {
    let t21175 = t6484 * t6373;
    let t21176 = 7.0 / 24.0 * t21175;
    let t21181 = -t2277 * t2255 * t6350 * t6598 / 256.0 - 35.0 / 96.0 * t21146 + t6637 * t20527 * t21148 / 32.0 + t21155 - t21158 - 7.0 / 64.0 * t21159 - t2277 * t2255 * t2278 * t21161 / 512.0 + 5.0 / 192.0 * t929 * t2300 * t904 * t19993 + t21174 + t21176 - t2343 * t3235 * t20264 * t875 / 384.0;
    (t21176, t21181)
}
