//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta59 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk392;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk393;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk394;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta59<F: Float>(t1132: F, t1134: F, t1118: F, t406: F, t281: F, t414: F, t926: F, t240: F, t462: F, t1122: F, t141: F, t1124: F, t421: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1135, t1137, t1139) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk392::<F>(t1132, t1134, t1118, t406);
        let (t1140, t1143, t1144, t1145) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk393::<F>(t1134, t1139, t281, t414, t926, t240, t462);
        let (t1146, t1147, t1149) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk394::<F>(t1122, t1145, t141, t1124, t1135, t1137, t1140, t1144);
        let t1150 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk395::<F>(t421);
    (t1135, t1137, t1139, t1140, t1143, t1144, t1145, t1146, t1147, t1149, t1150)
}
