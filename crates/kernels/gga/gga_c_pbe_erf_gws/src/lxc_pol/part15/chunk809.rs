//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 809/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk809<F: Float>(t2572: F, t4913: F, t2705: F, t422: F, t7194: F, t1620: F, t1812: F, t7527: F, t1882: F, t2790: F, t2660: F, t2796: F, t2800: F, t1879: F, t1033: F, t1726: F) -> (F, F, F, F, F, F, F, F) {
    let t7609 = 16.0 / 45.0 * t4913 * t2572;
    let t7610 = t2705 * t422;
    let t7611 = t7194 * t7610;
    let t7613 = 16.0 / 45.0 * t1620 * t7611;
    let t7615 = 16.0 / 45.0 * t7527 * t1812;
    let t7617 = 16.0 / 45.0 * t2790 * t1882;
    let t7619 = 16.0 / 45.0 * t2660 * t2796;
    let t7621 = 8.0 / 15.0 * t2660 * t2800;
    let t7623 = 16.0 / 45.0 * t1879 * t2796;
    let t7625 = 2.0 / 15.0 * t1033 * t1726;
    (t7609, t7613, t7615, t7617, t7619, t7621, t7623, t7625)
}
