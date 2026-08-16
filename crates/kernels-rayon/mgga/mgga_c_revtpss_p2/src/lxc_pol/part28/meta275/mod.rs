//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1231;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1232;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1233;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1234;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta275(t225: f64, t385: f64, t7810: f64, t1646: f64, t1976: f64, t7145: f64, t1651: f64, t1678: f64, t1982: f64, t1695: f64, t7160: f64, t1089: f64, t1668: f64, t7168: f64, t1984: f64, t359: f64, t1647: f64, t1652: f64, t1696: f64, t1978: f64, t1983: f64, t1986: f64, t342: f64, t7102: f64, t7140: f64, t7144: f64, t7151: f64, t7159: f64, t7167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7812, t7817) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1231(t225, t385, t7810, t1646, t1976);
        let t7818 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1232(t7145, t7817);
        let t7821 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1233(t1651, t1976);
        let (t7822, t7825, t7828) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1234(t7145, t7821, t1678, t1982, t1695, t1976);
        let (t7829, t7833, t7837, t7840) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1235(t7160, t7828, t1089, t1668, t7168, t1984, t359, t7810, t1647, t1652, t1696, t1978, t1983, t1986, t342, t7102, t7140, t7144, t7151, t7159, t7167, t7812, t7818, t7822, t7825);
    (t7812, t7817, t7818, t7821, t7822, t7825, t7828, t7829, t7833, t7837, t7840)
}
