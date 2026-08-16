//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1215;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1216;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1217;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1218;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta271(t114: f64, t651: f64, t7735: f64, t1513: f64, t6998: f64, t6997: f64, t508: f64, t1544: f64, t30: f64, t1963: f64, t1549: f64, t7025: f64, t1561: f64, t7038: f64, t1565: f64, t7045: f64, t7024: f64, t7032: f64, t7035: f64, t7042: f64, t225: f64, t1568: f64, t1955: f64, t1579: f64, t1949: f64, t7071: f64, t1558: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7737, t7741) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1215(t114, t651, t7735, t1513, t6998, t6997);
        let t7742 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1216(t508, t7741);
        let (t7744, t7749, t7750, t7759) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1217(t651, t7742, t1544, t30, t1963, t1549, t7025, t1561, t7038, t1565, t7045, t7024, t7032, t7035, t7042);
        let (t7760, t7766, t7769) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1218(t225, t7759, t1568, t1955, t1579, t1949);
        let (t7770, t7774) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1219(t7071, t7769, t1558, t1949, t231);
    (t7737, t7741, t7742, t7744, t7749, t7750, t7759, t7760, t7766, t7769, t7770, t7774)
}
