//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk530;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk531;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk532;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta83<F: Float>(t1121: F, t1469: F, t1120: F, t128: F, t1119: F, t422: F, t1118: F, t1132: F, t1139: F, t1145: F, t141: F, t1137: F, t1144: F, t1150: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1715 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk530::<F>(t1121, t1469);
        let (t1716, t1717, t1719, t1721, t1723) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk531::<F>(t1120, t1715, t128, t1119, t422, t1118);
        let (t1724, t1727, t1729, t1730, t1732, t1733) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk532::<F>(t1132, t1723, t1139, t1145, t1715, t141, t1137, t1144, t1717, t1150);
    (t1715, t1716, t1717, t1719, t1721, t1723, t1724, t1727, t1729, t1730, t1732, t1733)
}
