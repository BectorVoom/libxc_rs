//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 936/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk936<F: Float>(t2261: F, t8640: F, t2253: F, t8630: F, t70: F, t8639: F, t41: F, t2268: F, t8669: F, t8675: F, t8652: F, t8665: F) -> (F, F, F, F, F, F, F) {
    let t39439 = t8640 * t2261;
    let t39441 = t2253 * t8630;
    let t39447 = t8639 * t70;
    let t39448 = t41 * t39447;
    let t39449 = t39448 * t2268;
    let t39451 = t8675 * t8669;
    let t39453 = t8675 * t8652;
    let t39455 = t8675 * t8665;
    (t39439, t39441, t39448, t39449, t39451, t39453, t39455)
}
