//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 437/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk437<F: Float>(t245: F, t2542: F, t258: F, t2331: F, t2465: F, t247: F, t2470: F, t2527: F, t2570: F, t2617: F, t263: F, t719: F, t771: F, t1580: F, t21: F, t267: F, t363: F, t5: F, t776: F) -> (F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t2619 = t2542 * t258;
    let t2624 = -t2331 * t263 - t2465 * t263 - t247 * t2617 - 2.0 * t719 * t771 - 4.0 * t2470 - 2.0 * t2527 + 4.0 * t2570 + 2.0 * t2619;
    let t2635 = piecewise3(t246, 0.0, t5 * t2624 * t21 / 4.0 + t5 * t776 * t363 / 2.0 + t5 * t267 * t1580 / 4.0);
    (t2619, t2624, t2635)
}
