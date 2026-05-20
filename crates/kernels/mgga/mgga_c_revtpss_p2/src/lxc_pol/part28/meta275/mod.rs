//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1231;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1232;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1233;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1234;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1235;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta275<F: Float>(t225: F, t385: F, t7810: F, t1646: F, t1976: F, t7145: F, t1651: F, t1678: F, t1982: F, t1695: F, t7160: F, t1089: F, t1668: F, t7168: F, t1984: F, t359: F, t1647: F, t1652: F, t1696: F, t1978: F, t1983: F, t1986: F, t342: F, t7102: F, t7140: F, t7144: F, t7151: F, t7159: F, t7167: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7812, t7817) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1231::<F>(t225, t385, t7810, t1646, t1976);
        let t7818 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1232::<F>(t7145, t7817);
        let t7821 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1233::<F>(t1651, t1976);
        let (t7822, t7825, t7828) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1234::<F>(t7145, t7821, t1678, t1982, t1695, t1976);
        let (t7829, t7833, t7837, t7840) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1235::<F>(t7160, t7828, t1089, t1668, t7168, t1984, t359, t7810, t1647, t1652, t1696, t1978, t1983, t1986, t342, t7102, t7140, t7144, t7151, t7159, t7167, t7812, t7818, t7822, t7825);
    (t7812, t7817, t7818, t7821, t7822, t7825, t7828, t7829, t7833, t7837, t7840)
}
