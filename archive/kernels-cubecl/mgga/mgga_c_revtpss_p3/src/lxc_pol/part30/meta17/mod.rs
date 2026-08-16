//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta17 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk124;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk125;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk126;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk127;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta17<F: Float>(t273: F, t276: F, t279: F, t285: F, t293: F, t300: F, t302: F, t199: F, t240: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t307, t310, t311) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk124::<F>(t273, t276, t279, t285);
        let t315 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk125::<F>(t273);
        let (t320, t323, t324) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk126::<F>(t273, t276, t279, t285);
        let (t328, t330, t334, t335) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk127::<F>(t315, t324, t293, t300, t302, t311, t199, t240, zeta_threshold);
        let t336 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk128::<F>(t334, t335);
    (t307, t310, t311, t315, t320, t323, t324, t328, t330, t334, t335, t336)
}
