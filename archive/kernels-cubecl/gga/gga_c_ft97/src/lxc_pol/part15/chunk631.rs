//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 631/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk631<F: Float>(t1546: F, t4426: F, t89: F, t4432: F, t1597: F, t4441: F, t534: F, t408: F, t4491: F, t1710: F, t4474: F, t8051: F) -> (F, F, F, F, F, F, F) {
    let t15609 = t89 * t1546 * t4426;
    let t15612 = t89 * t1546 * t4432;
    let t15630 = t4441 * t1597;
    let t15680 = t534 * t15630;
    let t15706 = t408 * t4491;
    let t15712 = t1710 * t4474;
    let t15716 = t8051 * t4474;
    (t15609, t15612, t15630, t15680, t15706, t15712, t15716)
}
