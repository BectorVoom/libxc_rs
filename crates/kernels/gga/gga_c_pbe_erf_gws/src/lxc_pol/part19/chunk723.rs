//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 723/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk723<F: Float>(t1639: F, t56: F, t1672: F, t662: F, t211: F, t648: F, t618: F, t616: F, t1651: F, t197: F, t597: F, t1630: F, t649: F, t596: F, t1617: F, t732: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5089 = t56 * t1639;
    let t5102 = t1672 * t662;
    let t5103 = t211 * t5102;
    let t5108 = t648 * t648;
    let t5109 = 1.0 / t5108;
    let t5116 = t1672 * t618;
    let t5117 = t616 * t5116;
    let t5125 = t1651 * t197;
    let t5129 = t1651 * t597;
    let t5137 = t1630 * t649;
    let t5174 = t596 * t596;
    let t5175 = 1.0 / t5174;
    let t5205 = t732 * t1617;
    (t5089, t5103, t5109, t5117, t5125, t5129, t5137, t5175, t5205)
}
