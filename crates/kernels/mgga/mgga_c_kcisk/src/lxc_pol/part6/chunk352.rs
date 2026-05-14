//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 352/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk352<F: Float>(t2231: F, t470: F, t487: F, t1487: F, t2152: F, t382: F, t486: F, t2211: F, t467: F, t492: F, t500: F, t499: F, t498: F, t1504: F, t381: F, t493: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2262 = t470 * t2231;
    let t2263 = t487 * t2262;
    let t2264 = t1487 * t2263;
    let t2266 = t382 * t2152;
    let t2267 = t487 * t2266;
    let t2268 = t486 * t2267;
    let t2270 = t2211 * t467;
    let t2271 = t2270 * t492;
    let t2272 = t2271 * t500;
    let t2274 = t499 * t2231;
    let t2275 = t498 * t2274;
    let t2276 = t1504 * t2275;
    let t2278 = t381 * t2152;
    let t2279 = t498 * t2278;
    let t2280 = t493 * t2279;
    (t2263, t2264, t2267, t2268, t2270, t2271, t2272, t2275, t2276, t2279, t2280)
}
