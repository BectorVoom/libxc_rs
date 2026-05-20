//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta26 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk200;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk201;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk202;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk203;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk204;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta26<F: Float>(t30: F, t513: F, t153: F, t33: F, zeta_threshold: F, t162: F, t189: F, t512: F, t187: F, t199: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t514, t515, t516) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk200::<F>(t30, t513, t153, t33, zeta_threshold);
        let (t517, t519, t520) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk201::<F>(t33, t516, t153, t515, t162, zeta_threshold);
        let t521 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk202::<F>(t189, t520);
        let (t522, t524, t525, t527, t530) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk203::<F>(t30, t33, t512, t521, t187, t520, t513, t199, t516, zeta_threshold);
        let t531 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk204::<F>(t530);
    (t514, t516, t517, t519, t520, t521, t522, t524, t525, t527, t530, t531)
}
