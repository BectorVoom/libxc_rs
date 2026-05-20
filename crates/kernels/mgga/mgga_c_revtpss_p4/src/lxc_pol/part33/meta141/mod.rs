//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta141 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk752;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk753;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk754;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk755;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk756;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk757;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk758;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta141<F: Float>(t1247: F, t3704: F, t1032: F, t1204: F, t1246: F, t1234: F, t1260: F, t1209: F, t1284: F, t3624: F, t482: F, t66: F, t828: F, t1269: F, t460: F, t1275: F, t493: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3705, t3707, t3708, t3711) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk752::<F>(t1247, t3704, t1032, t1204, t1246, t1234, t1260);
        let t3717 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk753::<F>(t1209, t1284);
        let t3718 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk754::<F>(t3624, t3717);
        let t3719 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk755::<F>(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk756::<F>(t3719, t828);
        let (t3732, t3736) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk757::<F>(t1269, t460, t1275, t493);
        let t3737 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk758::<F>(t225, t3736);
        let t3746 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk759::<F>(t1204, t1284);
    (t3705, t3707, t3708, t3711, t3717, t3718, t3719, t3720, t3732, t3736, t3737, t3746)
}
