//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1079;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1080;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1081;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1082;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1083;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1084;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1085;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1086;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1087;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta162(t605: f64, t30: f64, t2257: f64, t3833: f64, t513: f64, t527: f64, zeta_threshold: f64, t1113: f64, t33: f64, t3351: f64, t516: f64, t162: f64, t187: f64, t2608: f64, t520: f64, t512: f64, t189: f64, t19: f64, t27: f64, t521: f64, t14: f64, t22: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3834 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1079(t605);
        let (t3840, t3841) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1080(t30, t2257, t3833, t3834, t513, t527, zeta_threshold);
        let t3842 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1081(t1113);
        let t3850 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1082(t33, t3351, t3841, t3842, t516, t162, t3840, zeta_threshold);
        let (t3852, t3853) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1083(t187, t3850, t2608, t520);
        let t3854 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1084(t3853, t512);
        let t3855 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1085(t189, t3850);
        let (t3856, t3857) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1086(t3855, t512, t19, t27);
        let t3859 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1087(t3857, t521);
        let t3860 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1088(t14, t22);
    (t3834, t3841, t3842, t3850, t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860)
}
