//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1191/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1191<F: Float>(t1452: F, t814: F, t2306: F, t3074: F, t339: F, t6104: F, t860: F, t6373: F, t6484: F, t19993: F, t20264: F, t20527: F, t21146: F, t21148: F, t21155: F, t21158: F, t21159: F, t2255: F, t2277: F, t2278: F, t2300: F, t2343: F, t3235: F, t6350: F, t6598: F, t6637: F, t875: F, t904: F, t929: F) -> (F, F, F) {
    let t21161 = t1452 * t814;
    let t21174 = t3074 * t2306 * t6104 * t339 * t860 / F::cast_from(24.0_f64);
    let t21175 = t6484 * t6373;
    let t21176 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t21175;
    let t21181 = -t2277 * t2255 * t6350 * t6598 / F::cast_from(256.0_f64) - F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t21146 + t6637 * t20527 * t21148 / F::cast_from(32.0_f64) + t21155 - t21158 - F::cast_from(7.0_f64) / F::cast_from(64.0_f64) * t21159 - t2277 * t2255 * t2278 * t21161 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t929 * t2300 * t904 * t19993 + t21174 + t21176 - t2343 * t3235 * t20264 * t875 / F::cast_from(384.0_f64);
    (t21174, t21176, t21181)
}
