//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1137/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1137<F: Float>(t20388: F, t2121: F, t337: F, t6645: F, t2387: F, t6187: F, t2138: F, t6293: F, t6402: F, t20366: F, t20367: F, t20370: F, t20371: F, t20377: F, t20381: F, t20385: F, t20386: F, t2312: F, t6609: F, t9482: F) -> (F, F, F) {
    let t20390 = t2121 * t337 * t20388;
    let t20392 = t6645 * t20390 / F::new(4.0);
    let t20393 = t2387 * t6187;
    let t20395 = t20393 * t2138 / F::new(12.0);
    let t20396 = t6402 * t6293;
    let t20398 = -t20366 + F::new(7.0) / F::new(48.0) * t20367 - t20370 - t2312 * t9482 * t6609 * t20371 / F::new(24.0) + t20377 + t20381 + t20385 + F::new(7.0) / F::new(96.0) * t20386 + t20392 - t20395 + F::new(7.0) / F::new(96.0) * t20396;
    (t20392, t20395, t20398)
}
