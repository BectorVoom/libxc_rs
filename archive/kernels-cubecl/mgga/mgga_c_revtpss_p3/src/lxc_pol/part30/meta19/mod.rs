//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta19 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk135;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk136;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk137;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk138;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk139;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk140;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk141;
use chunk7::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta19<F: Float>(t225: F, t293: F, t328: F, t330: F, t355: F, sigma0: F, t39: F, t40: F, rho0: F, t351: F, t335: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t357 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk135::<F>(t225, t293, t328, t330, t355);
        let (t358, t359) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk136::<F>(t357);
        let t360 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk137::<F>(sigma0);
        let t361 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk138::<F>(t359, t360);
        let (t362, t365) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk139::<F>(t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk140::<F>(t361, t365);
        let (t367, t368) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk141::<F>(t351, t366, t335);
        let t369 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk142::<F>(t368);
    (t357, t358, t359, t360, t361, t362, t365, t366, t367, t368, t369)
}
