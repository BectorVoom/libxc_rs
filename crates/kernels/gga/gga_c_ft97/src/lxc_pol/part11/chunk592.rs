//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 592/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk592<F: Float>(t634: F, t8640: F, t2253: F, t2277: F, t2261: F, t2284: F, t2259: F, t72: F, t7765: F, t3621: F, t7789: F, t422: F, t639: F, t1643: F, t643: F, t2265: F, t631: F, t8621: F, t8626: F, t8630: F, t8636: F) -> (F, F, F, F, F) {
    let t8641 = t8640 * t634;
    let t8643 = t2253 * t2277;
    let t8645 = t2253 * t2261;
    let t8647 = t2253 * t2284;
    let t8650 = t72 * t2259 * t7765;
    let t8652 = t3621 * t7789;
    let t8654 = t422 * t639;
    let t8655 = t1643 * t643;
    let t8656 = t8654 * t8655;
    let t8659 = 6.0 * t631 * t8621 - 9.0 / 2.0 * t631 * t8626 + t631 * t8630 / 6.0 + 2.0 / 27.0 * t631 * t8636 + 5.0 / 9.0 * t8641 - t8643 / 3.0 - t8645 / 9.0 + 3.0 * t8647 + t631 * t8650 - t2265 * t8652 - t2265 * t8656 / 3.0;
    (t8650, t8652, t8654, t8656, t8659)
}
