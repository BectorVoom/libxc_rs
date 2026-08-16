//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta106 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk698;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk699;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk700;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk701;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk702;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk703;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta106(t595: f64, t65: f64, t235: f64, t2710: f64, t826: f64, t232: f64, t821: f64, t239: f64, t820: f64, t231: f64, t159: f64, t243: f64, t216: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2712, t2713) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk698(t595, t65, t235);
        let (t2716, t2718) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk699(t2710, t2713, t826, t232, t821);
        let t2719 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk700(t235, t2718);
        let (t2721, t2723) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk701(t239, t2719, t820, t231);
        let (t2729, t2730) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk702(t159, t243, t216);
        let t2735 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk703(t2712, t785);
    (t2712, t2713, t2716, t2718, t2719, t2721, t2723, t2729, t2730, t2735)
}
