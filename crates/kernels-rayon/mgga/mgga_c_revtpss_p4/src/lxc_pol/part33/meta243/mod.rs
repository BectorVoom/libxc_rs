//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1083;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1084;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1085;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1086;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta243(t1774: f64, t1211: f64, t1828: f64, t1277: f64, t3579: f64, t5044: f64, t6423: f64, t6427: f64, t6431: f64) -> (f64, f64, f64, f64, f64, f64) {
        let t6573 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1083(t1774);
        let t6574 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1084(t1211, t6573);
        let (t6579, t6580) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1085(t1774, t1828, t1277);
        let t6587 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1086(t3579, t5044, t6423, t6427, t6431);
        let t6588 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1087(t1211, t6587);
    (t6573, t6574, t6579, t6580, t6587, t6588)
}
