//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 781/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk781<F: Float>(t267: F, t7114: F, t1791: F, t641: F, t1044: F, t1816: F, t1018: F, t1672: F, t185: F, t1627: F, t2667: F, t2674: F, t2680: F, t2789: F, t586: F, t1824: F) -> (F, F, F, F, F, F, F, F) {
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7117 = t7116 * t1044;
    let t7118 = t7117 * t1816;
    let t7120 = 16.0 / 45.0 * t7115 * t7118;
    let t7121 = t1672 * t1018;
    let t7122 = t185 * t7121;
    let t7123 = 4.0 / 135.0 * t7122;
    let t7125 = 8.0 / 45.0 * t1627 * t2667;
    let t7127 = 16.0 / 45.0 * t1627 * t2674;
    let t7129 = 8.0 / 27.0 * t1627 * t2680;
    let t7130 = t2789 * t586;
    let t7132 = 16.0 / 45.0 * t7130 * t1824;
    (t7115, t7120, t7123, t7125, t7127, t7129, t7130, t7132)
}
