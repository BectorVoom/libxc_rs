//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 796/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk796<F: Float>(t3205: F, t329: F, t838: F, t3209: F, t3214: F, t4414: F, t1164: F, t2242: F, t3123: F, t6184: F, t3133: F, t6183: F, t2134: F, t1133: F, t874: F, t3179: F, t6331: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8801 = t329 * t838 * t3205;
    let t8803 = 7.0 / 24.0 * t8801 * t3209;
    let t8810 = 7.0 / 72.0 * t4414 * t3214;
    let t8818 = t2242 * t1164;
    let t8823 = 7.0 / 144.0 * t3123 * t6184;
    let t8824 = t6183 * t3133;
    let t8826 = 7.0 / 144.0 * t2134 * t8824;
    let t8827 = t1133 * t874;
    let t8833 = t6331 * t3179;
    (t8801, t8803, t8810, t8818, t8823, t8824, t8826, t8827, t8833)
}
