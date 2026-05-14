//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 204/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk204<F: Float>(t544: F, t88: F, t37: F, t34: F, t67: F, t62: F, t520: F, t522: F, t526: F, t531: F) -> (F, F, F, F, F, F, F) {
    let t546 = 4.0 * t544 * t88;
    let t547 = 1.0 / t37;
    let t548 = t34 * t547;
    let t564 = t67 * t67;
    let t565 = 1.0 / t564;
    let t566 = t62 * t565;
    let t571 = -0.1176575e1 * t520 - 0.516475e0 * t522 - 0.2103875e0 * t526 - 0.104195e0 * t531;
    (t546, t547, t548, t564, t565, t566, t571)
}
