//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 904/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk904<F: Float>(t1217: F, t810: F, t1261: F, t2368: F, t1216: F, t308: F, t2372: F, t40: F, t1243: F, t1258: F, t1262: F, t2359: F, t2363: F, t295: F, t305: F, t6648: F, t803: F, t8316: F, t8319: F, t8320: F, t8323: F, t8326: F, t8329: F, t8337: F, t8340: F, t991: F, t997: F) -> (F, F, F, F) {
    let t8341 = t1217 * t810;
    let t8344 = t2368 * t1261;
    let t8347 = t308 * t1216;
    let t8350 = t2372 * t40;
    let t8353 = F::new(200.0) / F::new(27.0) * t1243 * t991 - F::new(100.0) / F::new(27.0) * t803 * t2359 - F::new(50.0) / F::new(9.0) * t803 * t2363 - F::new(10.0) / F::new(27.0) * t295 * t8316 + F::new(20.0) / F::new(9.0) * t8319 * t8320 + F::new(10.0) / F::new(9.0) * t295 * t8323 + F::new(5.0) / F::new(3.0) * t295 * t8326 - F::new(5.0) * t295 * t8329 - F::new(50.0) / F::new(27.0) * t997 * t1258 - F::new(25.0) / F::new(9.0) * t997 * t1262 - F::new(10.0) / F::new(27.0) * t305 * t8337 - F::new(20.0) / F::new(9.0) * t8340 * t8341 + F::new(10.0) / F::new(9.0) * t305 * t8344 - F::new(5.0) / F::new(3.0) * t305 * t8347 + F::new(5.0) * t305 * t8350 + t6648;
    (t8344, t8347, t8350, t8353)
}
