//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta109 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk745;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk746;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk747;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk748;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk749;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk750;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta109(t2652: f64, t857: f64, t2430: f64, t828: f64, t855: f64, t212: f64, t27: f64, t225: f64, t816: f64, t240: f64, t823: f64, t243: f64, t836: f64, t231: f64, t596: f64, t813: f64, t2482: f64, t849: f64, t136: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2653, t2656, t2659) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk745(t2652, t857, t2430, t828, t855, t212, t27);
        let t2661 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk746(t225, t2659, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk747(t240, t823);
        let (t2664, t2665, t2666, t2668) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk748(t243, t836, t231, t2662, t2661, t240, t596);
        let (t2672, t2674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk749(t243, t2668, t816, t813, t2482, t27, t849);
        let t2675 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk750(t136, t854);
    (t2653, t2656, t2659, t2661, t2662, t2664, t2665, t2666, t2668, t2672, t2674, t2675)
}
