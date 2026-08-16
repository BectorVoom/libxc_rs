//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta127 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk828;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk829;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk830;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk831;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk832;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk833;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk834;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk835;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta127(t1175: f64, t300: f64, t3356: f64, t1203: f64, t1208: f64, t487: f64, t1204: f64, t1207: f64, t458: f64, t456: f64, t1209: f64, t1269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3531 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk828(t1175, t300);
        let (t3546, t3555) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk829(t3356, t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk830(t3555, t487);
        let t3561 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk831(t1204, t487);
        let t3565 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk832(t1207, t458);
        let t3566 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk833(t3565, t456);
        let t3567 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk834(t3566, t487);
        let t3572 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk835(t1209, t1269);
    (t3531, t3546, t3555, t3556, t3561, t3565, t3566, t3567, t3572)
}
