//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1227;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1229;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta274(t7071: f64, t7769: f64, t1558: f64, t1949: f64, t231: f64, t7076: f64, t233: f64, t7759: f64, t1957: f64, t1580: f64, t1956: f64, t1959: f64, t213: f64, t257: f64, t7017: f64, t7020: f64, t7053: f64, t7062: f64, t7066: f64, t7070: f64, t7760: f64, t7766: f64, t892: f64, t1583: f64, t30: f64, t1468: f64, t1940: f64, t1963: f64, t2403: f64, t7091: f64, t7750: f64, t1659: f64, t1972: f64, t1656: f64, t1665: f64, t1671: f64, t1675: f64, t375: f64, t7110: f64, t7111: f64, t7117: f64, t7122: f64, t7130: f64, t7132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7770, t7774) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1227(t7071, t7769, t1558, t1949, t231);
        let (t7775, t7778, t7779, t7782) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228(t7076, t7774, t233, t7759, t1957, t1580, t1956, t1959, t213, t257, t7017, t7020, t7053, t7062, t7066, t7070, t7760, t7766, t7770);
        let t7783 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1229(t7782, t892);
        let (t7787, t7794, t7801, t7810) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1230(t1583, t30, t1468, t1940, t1963, t2403, t7091, t7750, t7783, t1659, t1972, t1656, t1665, t1671, t1675, t375, t7110, t7111, t7117, t7122, t7130, t7132);
    (t7770, t7774, t7775, t7778, t7779, t7782, t7783, t7787, t7794, t7801, t7810)
}
