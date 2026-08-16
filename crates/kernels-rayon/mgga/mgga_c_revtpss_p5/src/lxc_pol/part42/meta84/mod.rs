//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk494;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk495;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk496;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta84(t1161: f64, t1180: f64, t1721: f64, t1735: f64, t1737: f64, t1745: f64, t1750: f64, t1757: f64, t300: f64, t435: f64, t1179: f64, t1188: f64, t1756: f64, t1196: f64, t1201: f64, t1717: f64, t459: f64, t1212: f64, t1211: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1761, t1763, t1765) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk494(t1161, t1180, t1721, t1735, t1737, t1745, t1750, t1757, t300, t435, t1179, t1188, t1756);
        let (t1767, t1769, t1770) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk495(t1196, t1765, t1201, t1717, t459);
        let t1774 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk496(t1212, t1717);
        let t1775 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk497(t1211, t1774);
    (t1761, t1763, t1765, t1767, t1769, t1770, t1774, t1775)
}
