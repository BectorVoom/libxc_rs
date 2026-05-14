//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 871/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk871<F: Float>(t13854: F, t41: F, t470: F, t486: F, t494: F, t391: F, t79: F, t499: F, t493: F, t13949: F, t4204: F, t4203: F, t1505: F, t4181: F, t1504: F, t14345: F, t14348: F, t14351: F, t14354: F, t14357: F, t14359: F, t14361: F, t14363: F, t14368: F, t14371: F, t14377: F) -> (F, F, F, F, F) {
    let t14379 = t13854 * t41;
    let t14380 = t14379 * t470;
    let t14381 = t486 * t14380;
    let t14383 = t494 * t494;
    let t14386 = 1.0 / t391 / t14383 * t79;
    let t14387 = t14386 * t499;
    let t14388 = t493 * t14387;
    let t14390 = t4204 * t13949;
    let t14391 = t4203 * t14390;
    let t14393 = t4181 * t1505;
    let t14394 = t1504 * t14393;
    let t14396 = t14345 / 32.0 - t14348 / 192.0 - 19.0 / 36.0 * t14351 + t14354 / 4.0 - 3.0 / 128.0 * t14357 + t14359 / 8.0 - t14361 / 64.0 + t14363 - 3.0 / 8.0 * t14368 + 11.0 / 9.0 * t14371 + t14377 / 864.0 - 77.0 / 27.0 * t14381 + 209.0 / 216.0 * t14388 - t14391 / 8.0 + 19.0 / 48.0 * t14394;
    (t14381, t14388, t14391, t14394, t14396)
}
