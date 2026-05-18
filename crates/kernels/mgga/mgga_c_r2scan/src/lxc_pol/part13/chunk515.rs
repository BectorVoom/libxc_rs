//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 515/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk515<F: Float>(t2327: F, t2336: F, t1248: F, t990: F, t806: F, t298: F, t35: F, t1216: F, t1000: F, t1256: F, t810: F, t308: F) -> (F, F, F, F, F, F, F, F) {
    let t2337 = t2327 + t2336;
    let t2358 = t1248 * t990;
    let t2359 = t2358 * t806;
    let t2362 = t298 * t35;
    let t2363 = t2362 * t1216;
    let t2368 = t1256 * t1000;
    let t2369 = t2368 * t810;
    let t2372 = t308 * t35;
    (t2337, t2358, t2359, t2362, t2363, t2368, t2369, t2372)
}
