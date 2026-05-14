//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1120/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1120<F: Float>(t10157: F, t27836: F, t446: F, t713: F, t1882: F, t27480: F, t18: F, t2354: F, t24526: F, t3281: F, t1434: F, t2399: F, t6891: F, t109414: F, t13702: F, t96935: F) -> (F, F, F, F, F, F, F) {
    let t109467 = t446 * t10157 * t27836 * t713;
    let t109469 = t1882 * t27480;
    let t109470 = 4.0 * t109469;
    let t109473 = t3281 * t2354 * t24526 * t18;
    let t109476 = t1434 * t2399 * t6891;
    let t109477 = 2.0 / 9.0 * t109476;
    let t109479 = t109414 * t96935 * t13702;
    (t109467, t109469, t109470, t109473, t109476, t109477, t109479)
}
