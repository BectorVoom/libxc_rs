//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk694;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk695;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk696;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta105(t2689: f64, t810: f64, t775: f64, t854: f64, t236: f64, t807: f64, t21: f64, t65: f64, t64: f64, t159: f64, t222: f64, t794: f64, t798: f64, t802: f64, t234: f64, t2453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2691, t2693, t2694, t2695, t2698) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk694(t2689, t810, t775, t854, t236, t807, t21, t65);
        let t2699 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk695(t2698, t64);
        let (t2700, t2702, t2703) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk696(t159, t2699, t222, t794, t798);
        let (t2704, t2710) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk697(t2703, t802, t234, t2453);
    (t2691, t2693, t2694, t2695, t2698, t2699, t2700, t2702, t2703, t2704, t2710)
}
