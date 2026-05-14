//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 727/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk727<F: Float>(t336: F, t5450: F, t714: F, t1627: F, t1631: F, t155: F, t641: F, t644: F, t639: F, t1782: F, t586: F, t1749: F, t1630: F, t1634: F, t1639: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t5451 = t5450 * t336;
    let t5452 = t5451 * t714;
    let t5459 = t1627 * t1631;
    let t5463 = t155 * t641;
    let t5464 = t5463 * t644;
    let t5465 = t639 * t5464;
    let t5467 = t1782 * t586;
    let t5470 = t1749 * t586;
    let t5477 = t1630 * t1634;
    let t5478 = t639 * t5477;
    let t5480 = t9 * t1639;
    (t5452, t5459, t5463, t5465, t5467, t5470, t5478, t5480)
}
