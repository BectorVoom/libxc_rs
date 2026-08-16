//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk324;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk325;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta47(t1118: f64, t159: f64, t482: f64, t635: f64, t418: f64, t408: f64, t409: f64, t406: f64, t281: f64, t414: f64, t926: f64, t240: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1119, t1120) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk324(t1118, t159, t482);
        let t1121 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk325(t635);
        let (t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk326(t418, t408, t409, t1118, t406, t281, t414, t926, t240, t462);
    (t1119, t1120, t1121, t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145)
}
