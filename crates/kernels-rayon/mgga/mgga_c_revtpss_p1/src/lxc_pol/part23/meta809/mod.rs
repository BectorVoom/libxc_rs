//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta809 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2644;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2645;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2646;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2647;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2648;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2649;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2650;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2651;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta809(t18972: f64, t698: f64, t18943: f64, t689: f64, t18938: f64, t11144: f64, t5825: f64, t19006: f64, t18910: f64, t18914: f64, t18905: f64, t18927: f64, t11150: f64, t18931: f64, t18947: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63242, t63276) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2644(t18972, t698, t18943, t689);
        let t63278 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2645(t18938, t689);
        let (t63287, t63320, t63338) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2646(t11144, t5825, t19006, t698, t18910, t689);
        let t63340 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2647(t18914, t689);
        let t63342 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2648(t18905, t689);
        let t63361 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2649(t18927, t689);
        let (t63363, t63371) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2650(t11150, t5825, t18931, t689);
        let t63447 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2651(t18947, t689);
    (t63242, t63276, t63278, t63287, t63320, t63338, t63340, t63342, t63361, t63363, t63371, t63447)
}
