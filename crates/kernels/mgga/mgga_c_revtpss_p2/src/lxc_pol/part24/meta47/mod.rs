//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta47 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk324;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk325;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta47<F: Float>(t1118: F, t159: F, t482: F, t635: F, t418: F, t408: F, t409: F, t406: F, t281: F, t414: F, t926: F, t240: F, t462: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1119, t1120) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk324::<F>(t1118, t159, t482);
        let t1121 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk325::<F>(t635);
        let (t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk326::<F>(t418, t408, t409, t1118, t406, t281, t414, t926, t240, t462);
    (t1119, t1120, t1121, t1129, t1130, t1131, t1132, t1137, t1139, t1143, t1144, t1145)
}
