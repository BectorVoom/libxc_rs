//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1065;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1066;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1067;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta234<F: Float>(t1294: F, t1774: F, t1277: F, t3358: F, t3579: F, t5044: F, t5049: F, t5054: F, t5058: F, t1211: F, t1209: F, t1811: F, t1256: F, t1804: F, t1786: F, t1230: F, t1803: F, t225: F, t5216: F, t480: F, t1796: F, t3172: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5236, t5237) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1065::<F>(t1294, t1774, t1277);
        let t5245 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1066::<F>(t3358, t3579, t5044, t5049, t5054, t5058);
        let t5246 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1067::<F>(t1211, t5245);
        let (t5251, t5254, t5256, t5258, t5261, t5262, t5265) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1068::<F>(t1209, t1811, t1256, t1804, t1786, t1230, t1803, t225, t5216, t480, t1796, t3172);
    (t5236, t5237, t5245, t5246, t5251, t5254, t5256, t5258, t5261, t5262, t5265)
}
