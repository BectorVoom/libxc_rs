//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 844/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk844<F: Float>(t2261: F, t8640: F, t2253: F, t8630: F, t70: F, t8639: F, t41: F, t2268: F, t8669: F, t8675: F, t8652: F, t8665: F, t8686: F, t8672: F, t1647: F, t1651: F, t2258: F, t2265: F, t2266: F, t2282: F, t2294: F, t37357: F, t379: F, t631: F, t8634: F, t8680: F, t8709: F) -> (F, F) {
    let t39439 = t8640 * t2261;
    let t39441 = t2253 * t8630;
    let t39447 = t8639 * t70;
    let t39448 = t41 * t39447;
    let t39449 = t39448 * t2268;
    let t39451 = t8675 * t8669;
    let t39453 = t8675 * t8652;
    let t39455 = t8675 * t8665;
    let t39457 = t8675 * t8686;
    let t39471 = t8675 * t8672;
    let t39481 = 10.0 / 27.0 * t39439 - 4.0 / 9.0 * t39441 + 2.0 * t631 * t2258 * t8634 * t37357 - 40.0 / 9.0 * t39449 + 8.0 / 3.0 * t39451 + 8.0 / 3.0 * t39453 - 4.0 / 9.0 * t39455 - 16.0 / 3.0 * t39457 - 12.0 * t2265 * t8680 * t1647 * t2282 - 4.0 / 3.0 * t2265 * t2266 * t379 * t8709 - 2.0 * t2265 * t2266 * t1651 * t2294 + 8.0 / 3.0 * t39471 + 4.0 * t2265 * t2266 * t1647 * t2294 + 6.0 * t2265 * t8680 * t1651 * t2282;
    (t39448, t39481)
}
