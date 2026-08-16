//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 835/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk835<F: Float>(t197: F, t5293: F, t219: F, t641: F, t1639: F, t1642: F, t1697: F, t5212: F, t1802: F, t589: F, t617: F, t631: F) -> (F, F, F, F, F, F) {
    let t7435 = t5293 * t197;
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7491 = t7490 * t1642;
    let t7505 = t5212 * t1697;
    let t7514 = t589 * t1802;
    let t7631 = t617 * t631;
    (t7435, t7483, t7491, t7505, t7514, t7631)
}
