//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1220;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1221;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1222;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1223;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta272(t7076: f64, t7774: f64, t233: f64, t7759: f64, t1957: f64, t1580: f64, t1956: f64, t1959: f64, t213: f64, t257: f64, t7017: f64, t7020: f64, t7053: f64, t7062: f64, t7066: f64, t7070: f64, t7760: f64, t7766: f64, t7770: f64, t892: f64, t1583: f64, t30: f64, t1468: f64, t1940: f64, t1963: f64, t2403: f64, t7091: f64, t7750: f64, t1659: f64, t1972: f64, t1656: f64, t1665: f64, t1671: f64, t1675: f64, t375: f64, t7110: f64, t7111: f64, t7117: f64, t7122: f64, t7130: f64, t7132: f64, t225: f64, t385: f64, t1646: f64, t1976: f64, t7145: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7775, t7778, t7779, t7782) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1220(t7076, t7774, t233, t7759, t1957, t1580, t1956, t1959, t213, t257, t7017, t7020, t7053, t7062, t7066, t7070, t7760, t7766, t7770);
        let t7783 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1221(t7782, t892);
        let (t7787, t7794, t7801, t7810) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1222(t1583, t30, t1468, t1940, t1963, t2403, t7091, t7750, t7783, t1659, t1972, t1656, t1665, t1671, t1675, t375, t7110, t7111, t7117, t7122, t7130, t7132);
        let (t7812, t7817) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1223(t225, t385, t7810, t1646, t1976);
        let t7818 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1224(t7145, t7817);
    (t7775, t7778, t7779, t7782, t7783, t7787, t7794, t7801, t7810, t7812, t7817, t7818)
}
