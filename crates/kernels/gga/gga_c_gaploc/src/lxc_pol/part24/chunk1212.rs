//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1212/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1212<F: Float>(t204: F, t34378: F, t587: F, t10421: F, t21417: F, t30374: F, t30378: F, t30380: F, t30382: F, t34352: F, t34354: F, t34356: F, t34358: F, t34361: F, t34366: F, t34370: F, t34374: F, t34377: F) -> (F,) {
    let t34381 = 0.18404604457881959845e2 * t587 * t204 * t34378;
    let t34382 = t10421 * t21417;
    let t34383 = 0.59584149919750711116e-1 * t34382;
    let t34384 = t34352 + t34354 + t34356 + t34358 + t34361 + t34366 - t34370 + t34374 + t34377 - t34381 + t30374 - t30378 + t34383 - t30380 - t30382;
    (t34384,)
}
