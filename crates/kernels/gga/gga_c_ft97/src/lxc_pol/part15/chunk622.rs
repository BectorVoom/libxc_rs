//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 622/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk622<F: Float>(t1882: F, t5332: F, t5323: F, t5319: F, t5374: F, t870: F, t5315: F, t5419: F, t5381: F, t5403: F, t5399: F, t5395: F, t5327: F, t5311: F, t1240: F, t2766: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19318 = t1882 * t5332;
    let t19320 = t1882 * t5323;
    let t19322 = t1882 * t5319;
    let t19333 = t5374 * t870;
    let t19343 = t1882 * t5315;
    let t19387 = t1882 * t5419;
    let t19389 = t1882 * t5381;
    let t19449 = t1882 * t5403;
    let t19451 = t1882 * t5399;
    let t19453 = t1882 * t5395;
    let t19482 = t1882 * t5327;
    let t19484 = t1882 * t5311;
    let t19500 = t2766 * t1240;
    (t19318, t19320, t19322, t19333, t19343, t19387, t19389, t19449, t19451, t19453, t19482, t19484, t19500)
}
