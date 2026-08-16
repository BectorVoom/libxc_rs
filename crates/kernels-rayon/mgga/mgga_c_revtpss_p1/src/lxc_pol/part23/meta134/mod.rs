//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta134 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk876;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk877;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk878;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk879;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk880;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta134(t527: f64, t2608: f64, t520: f64, t512: f64, t19: f64, t27: f64, t521: f64, t14: f64, t22: f64, t583: f64, t588: f64, t1320: f64, t1333: f64, t123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3841 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk876(t527);
        let t3853 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk877(t2608, t520);
        let (t3854, t3857) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk878(t3853, t512, t19, t27);
        let (t3859, t3860) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk879(t3857, t521, t14, t22);
        let (t3862, t3863) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk880(t3860, t521, t583, t588);
        let (t3865, t3867, t3869) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk881(t3863, t521, t1320, t1333, t123, t520);
    (t3841, t3853, t3854, t3857, t3859, t3860, t3862, t3863, t3865, t3867, t3869)
}
