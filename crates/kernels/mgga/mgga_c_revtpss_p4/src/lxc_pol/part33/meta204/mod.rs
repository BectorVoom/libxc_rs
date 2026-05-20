//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta204 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk951;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk952;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk953;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk954;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta204<F: Float>(t3627: F, t471: F, t1715: F, t3626: F, t1227: F, t1261: F, t1266: F, t1808: F, t3625: F, t3647: F, t3686: F, t3705: F, t5373: F, t5379: F, t5381: F, t5384: F, t5386: F, t5391: F, t5397: F, t5402: F, t1247: F, t1252: F, t1797: F, t3708: F, t3711: F, t484: F, t5254: F, t5256: F, t5258: F, t5262: F, t5266: F, t5270: F, t5274: F, t5279: F, t5287: F, t5338: F, t5372: F, t225: F, t494: F, t1811: F, t460: F, t1214: F, t1828: F, t1277: F, t1294: F, t3737: F, t1284: F, t1770: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5405, t5406, t5407, t5410) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk951::<F>(t3627, t471, t1715, t3626, t1227, t1261, t1266, t1808, t3625, t3647, t3686, t3705, t5373, t5379, t5381, t5384, t5386, t5391, t5397, t5402);
        let t5412 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk952::<F>(t1247, t1252, t1261, t1797, t3708, t3711, t484, t5254, t5256, t5258, t5262, t5266, t5270, t5274, t5279, t5287, t5338, t5372, t5410);
        let (t5414, t5417, t5422, t5423) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk953::<F>(t225, t494, t5412, t1811, t460, t1214, t1828, t1277);
        let (t5428, t5429) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk954::<F>(t1294, t1828, t3737);
        let t5436 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk955::<F>(t1284, t1770);
    (t5405, t5406, t5407, t5412, t5414, t5417, t5422, t5423, t5428, t5429, t5436)
}
