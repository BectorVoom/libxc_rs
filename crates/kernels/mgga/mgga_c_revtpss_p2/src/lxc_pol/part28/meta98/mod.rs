//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta98 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk624;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk625;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk626;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk627;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk628;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta98<F: Float>(t606: F, t70: F, t2: F, t580: F, t17: F, t30: F, t33: F, zeta_threshold: F, t36: F, t607: F, t627: F, t362: F, t41: F, t47: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2251 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk624::<F>(t606);
        let (t2252, t2255) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk625::<F>(t2251, t70, t2, t580);
        let (t2256, t2257) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk626::<F>(t17, t2255);
        let t2258 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk627::<F>(t30, t33, t2257, zeta_threshold);
        let t2259 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk628::<F>(t2258, t36);
        let (t2260, t2263, t2269, t2270, t2275) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk629::<F>(t2259, t70, t607, t627, t362, t41, t47, sigma0);
    (t2251, t2252, t2255, t2256, t2257, t2258, t2259, t2260, t2263, t2269, t2270, t2275)
}
