//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1025;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1026;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1027;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1028;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1029;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1030;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta161(t1214: f64, t3759: f64, t1280: f64, t3584: f64, t3140: f64, t3596: f64, t460: f64, t3601: f64, t487: f64, t3303: f64, t3603: f64, t1248: f64, t1269: f64, t1287: f64, t3588: f64, t1243: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1025(t1214, t3759, t1280, t3584, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1026(t3766, t460);
        let (t3768, t3769) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1027(t3601, t487, t3303, t3603);
        let (t3770, t3774, t3778, t3781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1028(t3768, t3769, t1248, t1269, t1287, t3588, t487, t1243, t3140);
        let t3782 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1029(t3781, t460);
        let t3783 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1030(t3303, t471);
    (t3760, t3763, t3766, t3767, t3768, t3769, t3770, t3774, t3778, t3781, t3782, t3783)
}
