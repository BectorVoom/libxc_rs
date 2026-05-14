//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 666/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk666<F: Float>(t137: F, t5621: F, t510: F, t142: F, t1570: F, t2031: F, t168: F, t5589: F, t286: F, t159: F, t285: F, t4562: F, t4353: F, t4356: F, t4361: F, t4368: F, t4371: F, t4374: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5622 = t5621 * t137;
    let t5623 = t510 * t510;
    let t5624 = t142 * t5623;
    let t5625 = t5622 * t5624;
    let t5628 = t142 * t1570;
    let t5629 = t2031 * t5628;
    let t5631 = t168 * t5589;
    let t5633 = 0.19513566535229733338e0 * t5631 * t286;
    let t5636 = t4562 * t159 * t285;
    let t5645 = 4.0 / 27.0 * t4353 - t4356 / 3.0 + t4361 / 3.0 + 4.0 / 27.0 * t4368 - t4371 / 3.0 + t4374 / 3.0;
    (t5623, t5624, t5625, t5628, t5629, t5631, t5633, t5636, t5645)
}
