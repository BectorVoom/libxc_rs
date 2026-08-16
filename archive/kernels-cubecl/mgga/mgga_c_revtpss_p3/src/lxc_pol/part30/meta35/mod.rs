//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta35 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk235;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk236;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk237;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk238;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk239;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta35<F: Float>(t3: F, t65: F, t125: F, t123: F, t147: F, t143: F, t130: F, t131: F, t72: F, t122: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t675 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk235::<F>(t3, t65);
        let t676 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk236::<F>(t125, t675);
        let t679 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk237::<F>(t123, t147, t676);
        let (t680, t681, t682, t684, t685) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk238::<F>(t143, t130, t131, t72, t122, t125);
        let t686 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk239::<F>(t675, t685);
        let (t687, t689) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk240::<F>(t684, t686, t123, t676);
    (t675, t676, t679, t680, t681, t682, t684, t685, t686, t687, t689)
}
