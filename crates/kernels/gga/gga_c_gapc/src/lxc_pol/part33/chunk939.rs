//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 939/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk939<F: Float>(t11499: F, t185: F, t11496: F, t3116: F, t436: F, t3115: F, t11388: F, t3123: F, t1453: F, t474: F) -> (F, F, F, F, F, F) {
    let t11500 = t185 * t11499;
    let t11501 = t11500 * t11496;
    let t11503 = t436 * t3116;
    let t11504 = t3115 * t11503;
    let t11506 = t11388 * t3123;
    let t11508 = t474 * t1453;
    (t11500, t11501, t11503, t11504, t11506, t11508)
}
