//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 774/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk774<F: Float>(t253: F, t6365: F, t2182: F, t6: F, t875: F, t3139: F, t6269: F, t2168: F, t2171: F, t2345: F, t6308: F, t2277: F, t2312: F, t2343: F, t3247: F, t6316: F, t6321: F, t6324: F, t6330: F, t6334: F, t6338: F, t6344: F, t6347: F, t6352: F, t6357: F, t6362: F) -> (F, F, F, F, F, F, F) {
    let t6366 = t6365 * t253;
    let t6367 = t6 * t2182;
    let t6369 = t6366 * t6367 * t875;
    let t6373 = t3139 * t6269 * t875;
    let t6375 = t2168 * t6373 / F::new(32.0);
    let t6377 = t2345 * t6308 * t2171;
    let t6380 = t2343 * t6316 / F::new(128.0) - t6321 - t6324 + t6330 + t6334 - t6338 + t6344 - t2312 * t6347 / F::new(64.0) + t2277 * t6352 / F::new(256.0) + t2277 * t6357 / F::new(256.0) + F::new(3.0) / F::new(512.0) * t3247 * t6362 - F::new(5.0) / F::new(128.0) * t2343 * t6369 - t6375 + t2343 * t6377 / F::new(128.0);
    (t6366, t6367, t6369, t6373, t6375, t6377, t6380)
}
