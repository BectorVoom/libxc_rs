//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1654;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1655;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1656;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1657;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1658;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta439(t2251: f64, t5046: f64, t1120: f64, t128: f64, t3367: f64, t4186: f64, t606: f64, t2258: f64, t5051: f64, t1121: f64, t13312: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12610: f64, t16706: f64, t16708: f64, t16711: f64, t16713: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16733, t16735) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1654(t2251, t5046, t1120, t128);
        let (t16738, t16740) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1655(t3367, t4186, t606, t1120, t128);
        let (t16742, t16744) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1656(t2258, t5051, t1120, t128);
        let (t16746, t16748) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1657(t1121, t13312, t1120, t128);
        let t16750 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1658(t12297, t12299, t12301, t12303, t12610, t16706, t16708, t16711, t16713, t16717, t16722, t16727, t16731, t16735, t16740, t16744, t16748);
    (t16733, t16735, t16738, t16740, t16742, t16744, t16746, t16748, t16750)
}
