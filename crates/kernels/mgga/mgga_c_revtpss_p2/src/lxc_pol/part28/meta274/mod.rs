//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1227;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1229;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta274<F: Float>(t7071: F, t7769: F, t1558: F, t1949: F, t231: F, t7076: F, t233: F, t7759: F, t1957: F, t1580: F, t1956: F, t1959: F, t213: F, t257: F, t7017: F, t7020: F, t7053: F, t7062: F, t7066: F, t7070: F, t7760: F, t7766: F, t892: F, t1583: F, t30: F, t1468: F, t1940: F, t1963: F, t2403: F, t7091: F, t7750: F, t1659: F, t1972: F, t1656: F, t1665: F, t1671: F, t1675: F, t375: F, t7110: F, t7111: F, t7117: F, t7122: F, t7130: F, t7132: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7770, t7774) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1227::<F>(t7071, t7769, t1558, t1949, t231);
        let (t7775, t7778, t7779, t7782) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228::<F>(t7076, t7774, t233, t7759, t1957, t1580, t1956, t1959, t213, t257, t7017, t7020, t7053, t7062, t7066, t7070, t7760, t7766, t7770);
        let t7783 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1229::<F>(t7782, t892);
        let (t7787, t7794, t7801, t7810) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1230::<F>(t1583, t30, t1468, t1940, t1963, t2403, t7091, t7750, t7783, t1659, t1972, t1656, t1665, t1671, t1675, t375, t7110, t7111, t7117, t7122, t7130, t7132);
    (t7770, t7774, t7775, t7778, t7779, t7782, t7783, t7787, t7794, t7801, t7810)
}
