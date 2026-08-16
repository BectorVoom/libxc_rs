//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 774/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk774(t253: f64, t6365: f64, t2182: f64, t6: f64, t875: f64, t3139: f64, t6269: f64, t2168: f64, t2171: f64, t2345: f64, t6308: f64, t2277: f64, t2312: f64, t2343: f64, t3247: f64, t6316: f64, t6321: f64, t6324: f64, t6330: f64, t6334: f64, t6338: f64, t6344: f64, t6347: f64, t6352: f64, t6357: f64, t6362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6366 = t6365 * t253;
    let t6367 = t6 * t2182;
    let t6369 = t6366 * t6367 * t875;
    let t6373 = t3139 * t6269 * t875;
    let t6375 = t2168 * t6373 / 32.0_f64;
    let t6377 = t2345 * t6308 * t2171;
    let t6380 = t2343 * t6316 / 128.0_f64 - t6321 - t6324 + t6330 + t6334 - t6338 + t6344 - t2312 * t6347 / 64.0_f64 + t2277 * t6352 / 256.0_f64 + t2277 * t6357 / 256.0_f64 + 3.0_f64 / 512.0_f64 * t3247 * t6362 - 5.0_f64 / 128.0_f64 * t2343 * t6369 - t6375 + t2343 * t6377 / 128.0_f64;
    (t6366, t6367, t6369, t6373, t6375, t6377, t6380)
}
