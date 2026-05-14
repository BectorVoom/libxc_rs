//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 902/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk902<F: Float>(t12588: F, t572: F, t12537: F, t5283: F, t587: F, t12485: F, t586: F, t12452: F, t583: F, t12813: F, t5129: F, t12702: F, t185: F, t582: F, t3443: F, t995: F) -> (F, F, F, F, F, F, F) {
    let t40422 = t12588 * t572;
    let t40474 = t587 * t5283 * t12537;
    let t40493 = t12485 * t586;
    let t40498 = t12452 * t583;
    let t40527 = t587 * t5129 * t12813;
    let t40547 = t185 * t582 * t12702;
    let t40558 = t3443 * t995;
    (t40422, t40474, t40493, t40498, t40527, t40547, t40558)
}
