//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk539;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk540;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk541;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk542;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta85(t1131: f64, t1733: f64, t1154: f64, t1717: f64, t1163: f64, t1166: f64, t1724: f64, t1727: f64, t1730: f64, t1169: f64, t1173: f64, t448: f64, t1182: f64, t1185: f64, t1188: f64, t1161: f64, t1180: f64, t1721: f64, t300: f64, t435: f64, t1179: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1735, t1737, t1744, t1745) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk539(t1131, t1733, t1154, t1717, t1163, t1166, t1724, t1727, t1730, t1169);
        let t1749 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk540(t1173, t1717);
        let (t1750, t1756) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk541(t1749, t448, t1182, t1185, t1717, t1724, t1727, t1730);
        let t1757 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk542(t1188, t1756);
        let (t1761, t1763, t1765) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk543(t1161, t1180, t1721, t1735, t1737, t1745, t1750, t1757, t300, t435, t1179, t1188, t1756);
    (t1735, t1737, t1744, t1745, t1749, t1756, t1757, t1761, t1763, t1765)
}
