//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 808/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk808<F: Float>(t1802: F, t2784: F, t610: F, t1885: F, t587: F, t1635: F, t2612: F, t1645: F, t1656: F, t2615: F, t1666: F, t1010: F, t5406: F, t1648: F, t2622: F, t2572: F, t7011: F) -> (F, F, F, F, F, F, F, F) {
    let t7589 = t1802 * t2784;
    let t7590 = t7589 * t610;
    let t7591 = t1885 * t7590;
    let t7593 = 8.0 / 15.0 * t587 * t7591;
    let t7595 = 4.0 / 45.0 * t2612 * t1635;
    let t7597 = 4.0 / 27.0 * t2612 * t1645;
    let t7599 = 4.0 / 45.0 * t2615 * t1656;
    let t7601 = 4.0 / 27.0 * t2615 * t1666;
    let t7603 = 4.0 / 45.0 * t5406 * t1010;
    let t7605 = 16.0 / 45.0 * t1648 * t2622;
    let t7607 = 16.0 / 45.0 * t7011 * t2572;
    (t7593, t7595, t7597, t7599, t7601, t7603, t7605, t7607)
}
