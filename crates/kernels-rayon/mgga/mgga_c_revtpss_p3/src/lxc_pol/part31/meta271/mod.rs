//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1216;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1217;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1218;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1219;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta271(t651: f64, t7742: f64, t1518: f64, t2007: f64, t1544: f64, t30: f64, t1963: f64, t1549: f64, t7025: f64, t1561: f64, t7038: f64, t1565: f64, t7045: f64, t7024: f64, t7032: f64, t7035: f64, t7042: f64, t225: f64, t1568: f64, t1955: f64, t1579: f64, t1949: f64, t7071: f64, t1558: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7744, t7746) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1216(t651, t7742, t1518, t2007);
        let (t7749, t7750, t7759) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1217(t1544, t30, t1963, t1549, t7025, t1561, t7038, t1565, t7045, t7024, t7032, t7035, t7042);
        let (t7760, t7766, t7769) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1218(t225, t7759, t1568, t1955, t1579, t1949);
        let (t7770, t7774) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1219(t7071, t7769, t1558, t1949, t231);
    (t7744, t7746, t7749, t7750, t7759, t7760, t7766, t7769, t7770, t7774)
}
