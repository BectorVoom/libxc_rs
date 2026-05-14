//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1011/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1011<F: Float>(t20378: F, t2382: F, t6570: F, t19626: F, t346: F, t822: F, t6299: F, t6402: F, t20296: F, t2157: F, t2121: F, t337: F, t6645: F, t2387: F, t6187: F, t2138: F) -> (F, F, F, F, F) {
    let t20381 = 11.0 / 96.0 * t2382 * t20378 * t6570;
    let t20382 = t19626 * t346;
    let t20385 = t822 * t20382 * t6570 / 16.0;
    let t20386 = t6402 * t6299;
    let t20388 = t20296 * t2157;
    let t20390 = t2121 * t337 * t20388;
    let t20392 = t6645 * t20390 / 4.0;
    let t20393 = t2387 * t6187;
    let t20395 = t20393 * t2138 / 12.0;
    (t20381, t20385, t20386, t20392, t20395)
}
