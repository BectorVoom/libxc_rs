//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta179 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk899;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk900;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk901;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk902;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk903;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta179<F: Float>(t3906: F, t3908: F, t1420: F, t786: F, t1364: F, t1426: F, t556: F, t1444: F, t676: F, t123: F, t1363: F, t2470: F, t1362: F, t1398: F, t543: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3910, t3911, t3912, t3914, t3915) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk899::<F>(t3906, t3908, t1420, t786, t1364, t1426, t556);
        let t3916 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk900::<F>(t1444, t676);
        let t3917 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk901::<F>(t123, t3916);
        let (t3918, t3920) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk902::<F>(t3915, t3917, t1363, t2470);
        let (t3922, t3923) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk903::<F>(t1362, t3920, t1398);
        let t3924 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk904::<F>(t3923, t543);
    (t3910, t3911, t3912, t3914, t3915, t3916, t3917, t3918, t3920, t3922, t3923, t3924)
}
