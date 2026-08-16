//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta25 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk194;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk195;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk196;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk197;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk198;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta25<F: Float>(t117: F, t93: F, t19: F, t22: F, t30: F, t153: F, t33: F, zeta_threshold: F, t162: F, t189: F) -> (F, F, F, F, F, F, F, F, F) {
        let t511 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk194::<F>(t117, t93);
        let t512 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk195::<F>(t19, t22);
        let t513 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk196::<F>(t30);
        let (t514, t515, t516) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk197::<F>(t30, t513, t153, t33, zeta_threshold);
        let (t517, t519, t520) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk198::<F>(t33, t516, t153, t515, t162, zeta_threshold);
        let t521 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk199::<F>(t189, t520);
    (t511, t512, t513, t514, t516, t517, t519, t520, t521)
}
