//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 892/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk892<F: Float>(t2409: F, t3212: F, t6781: F, t3205: F, t329: F, t838: F, t3209: F, t2494: F, t810: F) -> (F, F, F, F) {
    let t8797 = t2409 * t6781 * t3212;
    let t8801 = t329 * t838 * t3205;
    let t8803 = 7.0 / 24.0 * t8801 * t3209;
    let t8804 = t2494 * t810;
    (t8797, t8801, t8803, t8804)
}
