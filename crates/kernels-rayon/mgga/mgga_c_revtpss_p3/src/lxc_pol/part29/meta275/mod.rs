//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1133;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1134;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1135;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1136;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta275(t7076: f64, t8011: f64, t233: f64, t7997: f64, t1957: f64, t1580: f64, t1956: f64, t2067: f64, t213: f64, t257: f64, t7070: f64, t7387: f64, t7390: f64, t7403: f64, t7409: f64, t7411: f64, t7766: f64, t7998: f64, t8007: f64, t892: f64, t30: f64, t265: f64, t393: f64, t1544: f64, t2071: f64, t207: f64, t1583: f64, t1940: f64, t198: f64, t2403: f64, t7432: f64, t1468: f64, t1469: f64, t2078: f64, t45: f64, t7787: f64, t7991: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t33: f64, t502: f64, t7862: f64, t1711: f64, t2085: f64, t57: f64, t7869: f64, rho1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8012, t8015, t8016, t8019) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1133(t7076, t8011, t233, t7997, t1957, t1580, t1956, t2067, t213, t257, t7070, t7387, t7390, t7403, t7409, t7411, t7766, t7998, t8007);
        let t8020 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1134(t8019, t892);
        let (t8031, t8039, t8040, t8045) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1135(t30, t265, t393, t1544, t2071, t207, t8019, t1583, t1940, t198, t2403, t7432, t892, t1468, t1469, t2078, t45, t7787, t7991, t8020, dens_threshold, rho0, zeta_threshold);
        let (t8046, t8059, t8064) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1136(t33, t265, t502, t2071, t7862, t8039, t1469, t1711, t1940, t2085, t2403, t57, t7432, t7869, t8020, dens_threshold, rho1, zeta_threshold);
        let t8065 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1137(t8045, t8064);
    (t8012, t8015, t8016, t8019, t8020, t8031, t8040, t8046, t8059, t8065)
}
