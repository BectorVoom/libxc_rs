//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1055/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1055<F: Float>(t2372: F, t40: F, t1243: F, t1258: F, t1262: F, t2359: F, t2363: F, t295: F, t305: F, t6648: F, t803: F, t8316: F, t8319: F, t8320: F, t8323: F, t8326: F, t8329: F, t8337: F, t8340: F, t8341: F, t8344: F, t8347: F, t991: F, t997: F) -> (F, F) {
    let t8350 = t2372 * t40;
    let t8353 = 200.0 / 27.0 * t1243 * t991 - 100.0 / 27.0 * t803 * t2359 - 50.0 / 9.0 * t803 * t2363 - 10.0 / 27.0 * t295 * t8316 + 20.0 / 9.0 * t8319 * t8320 + 10.0 / 9.0 * t295 * t8323 + 5.0 / 3.0 * t295 * t8326 - 5.0 * t295 * t8329 - 50.0 / 27.0 * t997 * t1258 - 25.0 / 9.0 * t997 * t1262 - 10.0 / 27.0 * t305 * t8337 - 20.0 / 9.0 * t8340 * t8341 + 10.0 / 9.0 * t305 * t8344 - 5.0 / 3.0 * t305 * t8347 + 5.0 * t305 * t8350 + t6648;
    (t8350, t8353)
}
