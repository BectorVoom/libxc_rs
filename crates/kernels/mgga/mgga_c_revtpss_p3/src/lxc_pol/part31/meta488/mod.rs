//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta488 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1782;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1783;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1784;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1785;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1786;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta488<F: Float>(t1971: F, t3104: F, t351: F, t25516: F, t3114: F, t3057: F, t7143: F, t1035: F, t8515: F, t1983: F, t378: F, t7150: F, t8521: F, t995: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25576, t25577, t25580) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1782::<F>(t1971, t3104, t351, t25516, t3114);
        let t25591 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1783::<F>(t3057, t7143);
        let t25604 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1784::<F>(t1035, t8515);
        let t25605 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1785::<F>(t1983, t25604);
        let (t25610, t25611) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1786::<F>(t378, t7150, t8521);
        let t25629 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1787::<F>(t8521, t995);
    (t25576, t25577, t25580, t25591, t25604, t25605, t25610, t25611, t25629)
}
