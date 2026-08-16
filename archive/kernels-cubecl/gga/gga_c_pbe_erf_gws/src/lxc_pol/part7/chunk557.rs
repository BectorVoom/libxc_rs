//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 557/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk557<F: Float>(t331: F, t641: F, t589: F, t181: F, t562: F, t184: F, t1640: F, t219: F) -> (F, F, F, F, F) {
    let t2591 = t331 * t641;
    let t2620 = t331 * t589;
    let t2659 = t562 * t181;
    let t2660 = t2659 * t184;
    let t2677 = t1640 * t219;
    (t2591, t2620, t2659, t2660, t2677)
}
