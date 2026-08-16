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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk752;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk753;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk754;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk755;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk756;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk757;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk758;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk759;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta141(t1247: f64, t3704: f64, t1032: f64, t1204: f64, t1246: f64, t1234: f64, t1260: f64, t1209: f64, t1284: f64, t3624: f64, t482: f64, t66: f64, t828: f64, t1269: f64, t460: f64, t1275: f64, t493: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3705, t3707, t3708, t3711) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk752(t1247, t3704, t1032, t1204, t1246, t1234, t1260);
        let t3717 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk753(t1209, t1284);
        let t3718 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk754(t3624, t3717);
        let t3719 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk755(t482, t66);
        let t3720 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk756(t3719, t828);
        let (t3732, t3736) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk757(t1269, t460, t1275, t493);
        let t3737 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk758(t225, t3736);
        let t3746 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk759(t1204, t1284);
    (t3705, t3707, t3708, t3711, t3717, t3718, t3719, t3720, t3732, t3736, t3737, t3746)
}
