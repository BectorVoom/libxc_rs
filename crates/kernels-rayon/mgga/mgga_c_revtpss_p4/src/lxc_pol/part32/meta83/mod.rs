//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk511;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk512;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta83(t30: f64, t1468: f64, t1469: f64, t1587: f64, t1704: f64, t265: f64, t395: f64, t45: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1121: f64, t1120: f64, t128: f64, t1119: f64, t422: f64, t1118: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1709, t1711) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk511(t30, t1468, t1469, t1587, t1704, t265, t395, t45, dens_threshold, rho0, zeta_threshold);
        let t1715 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk512(t1121, t1469);
        let (t1716, t1717, t1719, t1721, t1723) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk513(t1120, t1715, t128, t1119, t422, t1118);
    (t1709, t1711, t1715, t1716, t1717, t1719, t1721, t1723)
}
