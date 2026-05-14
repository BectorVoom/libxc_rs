//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1008/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1008<F: Float>(t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23576: F, t22028: F, t769: F, t25: F, t794: F) -> (F, F, F) {
    let t23578 = -0.27366666666666666666e-2 * t23543 - 0.6568e-2 * t23545 + 0.6568e-2 * t23551 + 0.14595555555555555556e-1 * t23553 + 0.1642e-1 * t23555 + 0.19704e-1 * t23557 - 0.14778e-1 * t23561 - 0.12315e-2 * t23565 + 0.3284e-2 * t23567 + 0.14595555555555555556e-2 * t23569 - 0.12771111111111111111e-2 * t23576;
    let t23579 = t769 * t22028;
    let t23581 = t25 * t794 * t23579;
    (t23578, t23579, t23581)
}
