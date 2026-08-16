//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta121 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk784;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk785;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk786;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk787;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk788;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk789;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk790;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta121(t1054: f64, t1058: f64, t1014: f64, t2857: f64, t1010: f64, t614: f64, t1016: f64, t140: f64, t1011: f64, t271: f64, t905: f64, t2852: f64, t1071: f64, t342: f64, t1077: f64, t384: f64, t225: f64, t1086: f64, t989: f64, t378: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3234, t3236, t3241) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk784(t1054, t1058, t1014, t2857, t1010, t614);
        let (t3245, t3252) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk785(t1016, t140, t1011, t271, t905);
        let (t3253, t3264) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk786(t2852, t3252, t1071, t342);
        let t3268 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk787(t1077, t384);
        let t3269 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk788(t225, t3268);
        let t3278 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk789(t1086, t989);
        let t3286 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk790(t1086, t378);
        let t3287 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk791(t3286, t994);
    (t3234, t3236, t3241, t3245, t3252, t3253, t3264, t3268, t3269, t3278, t3286, t3287)
}
