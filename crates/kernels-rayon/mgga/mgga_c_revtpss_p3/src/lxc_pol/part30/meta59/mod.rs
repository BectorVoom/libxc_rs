//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta59 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk383;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk384;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk385;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk386;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta59(t1132: f64, t1134: f64, t1118: f64, t406: f64, t281: f64, t414: f64, t926: f64, t240: f64, t462: f64, t1122: f64, t141: f64, t1124: f64, t421: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1135, t1137, t1139) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk383(t1132, t1134, t1118, t406);
        let (t1140, t1143, t1144, t1145) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk384(t1134, t1139, t281, t414, t926, t240, t462);
        let (t1146, t1147, t1149) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk385(t1122, t1145, t141, t1124, t1135, t1137, t1140, t1144);
        let t1150 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk386(t421);
    (t1135, t1137, t1139, t1140, t1143, t1144, t1145, t1146, t1147, t1149, t1150)
}
