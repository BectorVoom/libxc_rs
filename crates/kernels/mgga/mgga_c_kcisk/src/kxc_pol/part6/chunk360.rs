//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 360/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk360<F: Float>(t2271: F, t500: F, t2231: F, t499: F, t498: F, t1504: F, t2152: F, t381: F, t493: F, t2260: F, t2264: F, t2268: F) -> (F, F, F, F, F, F) {
    let t2272 = t2271 * t500;
    let t2274 = t499 * t2231;
    let t2275 = t498 * t2274;
    let t2276 = t1504 * t2275;
    let t2278 = t381 * t2152;
    let t2279 = t498 * t2278;
    let t2280 = t493 * t2279;
    let t2282 = t2260 / F::new(16.0) - t2264 / F::new(16.0) + t2268 / F::new(24.0) - t2272 / F::new(256.0) + t2276 / F::new(256.0) - t2280 / F::new(192.0);
    (t2272, t2275, t2276, t2279, t2280, t2282)
}
