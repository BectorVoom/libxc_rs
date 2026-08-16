//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1216;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1217;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1218;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1219;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta275(t1937: f64, t7732: f64, t1843: f64, t1936: f64, t114: f64, t651: f64, t1513: f64, t6998: f64, t6997: f64, t508: f64, t1544: f64, t30: f64, t1963: f64, t1549: f64, t7025: f64, t1561: f64, t7038: f64, t1565: f64, t7045: f64, t7024: f64, t7032: f64, t7035: f64, t7042: f64, t225: f64, t1568: f64, t1955: f64, t1579: f64, t1949: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7734, t7735) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1216(t1937, t7732, t1843, t1936);
        let (t7737, t7741) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1217(t114, t651, t7735, t1513, t6998, t6997);
        let t7742 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1218(t508, t7741);
        let (t7744, t7749, t7750, t7759) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1219(t651, t7742, t1544, t30, t1963, t1549, t7025, t1561, t7038, t1565, t7045, t7024, t7032, t7035, t7042);
        let (t7760, t7766, t7769) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1220(t225, t7759, t1568, t1955, t1579, t1949);
    (t7734, t7735, t7737, t7741, t7742, t7744, t7749, t7750, t7759, t7760, t7766, t7769)
}
