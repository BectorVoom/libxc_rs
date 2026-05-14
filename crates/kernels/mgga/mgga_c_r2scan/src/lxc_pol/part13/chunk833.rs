//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 833/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk833<F: Float>(t1216: F, t298: F, t2362: F, t40: F, t1000: F, t6635: F, t1257: F, t1256: F, t305: F, t1217: F, t810: F, t1261: F, t2368: F, t308: F, t2372: F, t1243: F, t1258: F, t1262: F, t2359: F, t2363: F, t295: F, t6648: F, t803: F, t8316: F, t8319: F, t8320: F, t8323: F, t991: F, t997: F) -> (F, F, F, F, F, F, F) {
    let t8326 = t298 * t1216;
    let t8329 = t2362 * t40;
    let t8336 = t6635 * t1000;
    let t8337 = t8336 * t1257;
    let t8340 = t305 * t1256;
    let t8341 = t1217 * t810;
    let t8344 = t2368 * t1261;
    let t8347 = t308 * t1216;
    let t8350 = t2372 * t40;
    let t8353 = 200.0 / 27.0 * t1243 * t991 - 100.0 / 27.0 * t803 * t2359 - 50.0 / 9.0 * t803 * t2363 - 10.0 / 27.0 * t295 * t8316 + 20.0 / 9.0 * t8319 * t8320 + 10.0 / 9.0 * t295 * t8323 + 5.0 / 3.0 * t295 * t8326 - 5.0 * t295 * t8329 - 50.0 / 27.0 * t997 * t1258 - 25.0 / 9.0 * t997 * t1262 - 10.0 / 27.0 * t305 * t8337 - 20.0 / 9.0 * t8340 * t8341 + 10.0 / 9.0 * t305 * t8344 - 5.0 / 3.0 * t305 * t8347 + 5.0 * t305 * t8350 + t6648;
    (t8326, t8329, t8337, t8344, t8347, t8350, t8353)
}
