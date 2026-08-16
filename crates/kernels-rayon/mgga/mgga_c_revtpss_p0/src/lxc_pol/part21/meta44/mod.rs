//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta44 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk330;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk331;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk332;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk333;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk334;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk335;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk336;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta44(t837: f64, t879: f64, t234: f64, t860: f64, t213: f64, t820: f64, t873: f64, t878: f64, t868: f64, t783: f64, t791: f64, t862: f64, t865: f64, t261: f64, t198: f64, t207: f64, t679: f64, t704: f64, t709: f64, t718: f64, t751: f64, t754: f64, t759: f64, t764: f64, t765: f64, t775: f64, t159: f64, t675: f64, t268: f64, t271: f64, t373: f64, t631: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t880, t883, t886) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk330(t837, t879, t234, t860, t213, t820, t873, t878);
        let t887 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk331(t868, t886);
        let t890 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk332(t213, t783, t791, t862, t865, t887);
        let t892 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk333(t261);
        let t895 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk334(t198, t207, t679, t704, t709, t718, t751, t754, t759, t764, t765, t775, t890, t892);
        let (t900, t902) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk335(t159, t675, t268, t271);
        let (t903, t904) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk336(t902, t159, t373);
        let t905 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk337(t631);
    (t880, t883, t886, t887, t890, t892, t895, t900, t902, t903, t904, t905)
}
