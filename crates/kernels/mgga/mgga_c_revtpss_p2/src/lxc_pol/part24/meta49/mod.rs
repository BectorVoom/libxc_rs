//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta49 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk330;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk331;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk332;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk333;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk334;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta49<F: Float>(t1179: F, t439: F, t1118: F, t1143: F, t447: F, t300: F, t458: F, t456: F, t487: F, t225: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1180, t1182, t1185, t1188) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk330::<F>(t1179, t439, t1118, t1143, t447);
        let t1196 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk331::<F>(t300, t439);
        let (t1201, t1207, t1208, t1209) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk332::<F>(t1118, t458, t456);
        let t1210 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk333::<F>(t1209, t487);
        let t1211 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk334::<F>(t225, t494);
    (t1180, t1182, t1185, t1188, t1196, t1201, t1207, t1208, t1209, t1210, t1211)
}
