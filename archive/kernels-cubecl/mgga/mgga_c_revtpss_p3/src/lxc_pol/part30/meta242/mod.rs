//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1080;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1081;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1082;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta242<F: Float>(t475: F, t5389: F, t467: F, t1264: F, t5056: F, t247: F, t3629: F, t5351: F, t3626: F, t3627: F, t471: F, t1715: F, t1227: F, t1261: F, t1266: F, t1808: F, t3625: F, t3647: F, t3686: F, t3705: F, t5373: F, t5379: F, t5381: F, t5384: F, t5386: F, t1247: F, t1252: F, t1797: F, t3708: F, t3711: F, t484: F, t5254: F, t5256: F, t5258: F, t5262: F, t5266: F, t5270: F, t5274: F, t5279: F, t5287: F, t5338: F, t5372: F, t225: F, t494: F, t1811: F, t460: F, t1214: F, t1828: F, t1277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5390, t5391, t5397, t5401, t5402, t5405, t5406) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1080::<F>(t475, t5389, t467, t1264, t5056, t247, t3629, t5351, t3626, t3627, t471, t1715);
        let (t5407, t5410) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1081::<F>(t3626, t5406, t1227, t1261, t1266, t1808, t3625, t3647, t3686, t3705, t5373, t5379, t5381, t5384, t5386, t5391, t5397, t5402);
        let t5412 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1082::<F>(t1247, t1252, t1261, t1797, t3708, t3711, t484, t5254, t5256, t5258, t5262, t5266, t5270, t5274, t5279, t5287, t5338, t5372, t5410);
        let (t5414, t5417, t5423) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1083::<F>(t225, t494, t5412, t1811, t460, t1214, t1828, t1277);
    (t5390, t5391, t5397, t5401, t5402, t5405, t5406, t5407, t5412, t5414, t5417, t5423)
}
