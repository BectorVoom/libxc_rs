//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1249/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1249<F: Float>(t23381: F, t295: F, t1243: F, t19093: F, t23366: F, t23376: F, t23409: F, t23411: F, t23413: F, t2359: F, t2363: F, t305: F, t6611: F, t6637: F, t6645: F, t803: F, t8316: F, t8326: F, t8329: F, t991: F, t997: F) -> (F,) {
    let t23443 = 20.0 * t295 * t23381;
    let t23448 = 10.0 * t305 * t23413 - 2200.0 / 81.0 * t6611 * t991 - 25.0 / 3.0 * t803 * t8326 - 10.0 * t295 * t23409 + 50.0 / 81.0 * t997 * t6637 - 25.0 / 9.0 * t997 * t6645 - t19093 + 10.0 / 9.0 * t305 * t23411 + 50.0 / 27.0 * t803 * t8316 + 25.0 * t803 * t8329 + 40.0 / 81.0 * t295 * t23366 + 10.0 / 3.0 * t295 * t23376 + t23443 + 400.0 / 27.0 * t1243 * t2359 + 200.0 / 9.0 * t1243 * t2363;
    (t23448,)
}
