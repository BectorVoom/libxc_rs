//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta273 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1225;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1226;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1227;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1228;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta273(t1651: f64, t1976: f64, t7145: f64, t1678: f64, t1982: f64, t1695: f64, t7160: f64, t1089: f64, t1668: f64, t7168: f64, t1984: f64, t359: f64, t7810: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7140: f64, t7144: f64, t7151: f64, t7159: f64, t7167: f64, t7812: f64, t7818: f64, t265: f64, t393: f64, t1544: f64, t1963: f64, t207: f64, t7782: f64, t1583: f64, t1940: f64, t198: f64, t2403: f64, t7091: f64, t892: f64, t1102: f64, t1699: f64, t336: f64, t5023: f64, t7181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t7821 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1225(t1651, t1976);
        let (t7822, t7825) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1226(t7145, t7821, t1678, t1982);
        let t7828 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1227(t1695, t1976);
        let (t7829, t7833, t7837, t7840) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1228(t7160, t7828, t1089, t1668, t7168, t1984, t359, t7810, t1647, t1652, t1696, t1978, t1983, t1986, t342, t7102, t7140, t7144, t7151, t7159, t7167, t7812, t7818, t7822, t7825);
        let (t7850, t7855, t7856) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1229(t265, t393, t1544, t1963, t207, t7782, t1583, t1940, t198, t2403, t7091, t892, t1102, t1699, t336, t5023, t7181, t7840);
    (t7821, t7822, t7825, t7828, t7829, t7833, t7837, t7840, t7850, t7855, t7856)
}
