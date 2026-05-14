//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 567/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk567<F: Float>(t1248: F, t990: F, t806: F, t298: F, t35: F, t1216: F, t1000: F, t1256: F, t810: F, t308: F, t1268: F, t295: F, t305: F, t803: F, t811: F, t991: F, t997: F) -> (F, F, F, F, F, F, F) {
    let t2358 = t1248 * t990;
    let t2359 = t2358 * t806;
    let t2362 = t298 * t35;
    let t2363 = t2362 * t1216;
    let t2368 = t1256 * t1000;
    let t2369 = t2368 * t810;
    let t2372 = t308 * t35;
    let t2373 = t2372 * t1216;
    let t2376 = -25.0 / 9.0 * t803 * t991 + 10.0 / 9.0 * t295 * t2359 + 5.0 / 3.0 * t295 * t2363 - 25.0 / 9.0 * t997 * t811 + 10.0 / 9.0 * t305 * t2369 - 5.0 / 3.0 * t305 * t2373 - t1268;
    (t2358, t2359, t2363, t2368, t2369, t2373, t2376)
}
