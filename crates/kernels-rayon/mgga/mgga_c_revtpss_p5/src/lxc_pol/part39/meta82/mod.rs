//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta82 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk480;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk481;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk482;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta82(t1598: f64, t1612: f64, t1614: f64, t1622: f64, t1627: f64, t1634: f64, t300: f64, t311: f64, t946: f64, t965: f64, t1633: f64, t964: f64, t973: f64, t981: f64, t1594: f64, t986: f64, t341: f64, t997: f64, t996: f64, t1015: f64, t1469: f64, t1012: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1638, t1640, t1642) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk480(t1598, t1612, t1614, t1622, t1627, t1634, t300, t311, t946, t965, t1633, t964, t973);
        let (t1644, t1646, t1647) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk481(t1642, t981, t1594, t986, t341);
        let t1651 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk482(t1594, t997);
        let (t1652, t1655, t1656, t1659) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk483(t1651, t996, t1015, t1469, t1012, t1647, t225);
    (t1638, t1640, t1642, t1644, t1646, t1647, t1651, t1652, t1655, t1656, t1659)
}
