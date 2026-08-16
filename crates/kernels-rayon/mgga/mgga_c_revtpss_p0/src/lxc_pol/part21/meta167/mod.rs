//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1058;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1059;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1060;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1061;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta167(t3906: f64, t3908: f64, t1420: f64, t786: f64, t1364: f64, t1426: f64, t556: f64, t1444: f64, t676: f64, t123: f64, t1363: f64, t2470: f64, t1362: f64, t1398: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3910, t3911, t3912, t3914, t3915) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1058(t3906, t3908, t1420, t786, t1364, t1426, t556);
        let (t3916, t3917) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1059(t1444, t676, t123);
        let (t3918, t3920) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1060(t3915, t3917, t1363, t2470);
        let (t3922, t3923) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1061(t1362, t3920, t1398);
        let t3924 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1062(t3923, t543);
    (t3910, t3911, t3912, t3914, t3915, t3916, t3917, t3918, t3920, t3922, t3923, t3924)
}
