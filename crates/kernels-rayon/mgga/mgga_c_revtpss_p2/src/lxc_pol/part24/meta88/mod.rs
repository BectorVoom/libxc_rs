//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk518;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk519;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk520;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk521;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk522;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta88(t159: f64, t2699: f64, t222: f64, t794: f64, t798: f64, t234: f64, t2453: f64, t595: f64, t65: f64, t235: f64, t826: f64, t232: f64, t821: f64, t239: f64, t820: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2700, t2702, t2703, t2710) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk518(t159, t2699, t222, t794, t798, t234, t2453);
        let (t2712, t2713) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk519(t595, t65, t235);
        let (t2716, t2718) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk520(t2710, t2713, t826, t232, t821);
        let t2719 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk521(t235, t2718);
        let (t2721, t2723) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk522(t239, t2719, t820, t231);
    (t2700, t2702, t2703, t2710, t2712, t2713, t2716, t2718, t2719, t2721, t2723)
}
