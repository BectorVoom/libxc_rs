//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 767/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk767<F: Float>(t219: F, t641: F, t1639: F, t5219: F, t995: F, t5212: F, t626: F, t1697: F, t1802: F, t589: F, t1631: F, t2612: F, t2740: F, t586: F, t2636: F, t5018: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7495 = t5219 * t995;
    let t7499 = t5212 * t626;
    let t7505 = t5212 * t1697;
    let t7514 = t589 * t1802;
    let t7526 = 16.0 / 135.0 * t2612 * t1631;
    let t7527 = t2740 * t586;
    let t7530 = t5018 * t2636;
    (t7483, t7490, t7495, t7499, t7505, t7514, t7526, t7527, t7530)
}
