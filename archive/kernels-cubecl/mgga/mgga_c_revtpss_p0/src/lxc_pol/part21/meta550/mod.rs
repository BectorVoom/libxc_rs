//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2224;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2225;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2226;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2227;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2228;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta550<F: Float>(t17240: F, t5052: F, t1222: F, t16738: F, t5308: F, t16742: F, t16733: F, t16771: F, t247: F, t3719: F, t3636: F, t5391: F, t5381: F, t1260: F, t12966: F, t16775: F, t1261: F, t17232: F, t17237: F, t5384: F, t5386: F, t16750: F, t482: F, t371: F, t372: F, t1803: F, t3666: F, t1208: F, t5215: F, t225: F, t480: F, t3678: F, t5327: F, t5323: F, t1235: F, t1238: F, t12800: F, t12976: F, t1791: F, t1808: F, t3644: F, t3663: F, t3667: F, t5320: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17243, t17244, t17247, t17250, t17254, t17258) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2224::<F>(t17240, t5052, t1222, t16738, t5308, t16742, t16733, t16771, t247, t3719, t3636, t5391);
        let (t17260, t17261) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2225::<F>(t3636, t5381, t1260, t12966);
        let (t17265, t17268) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2226::<F>(t16775, t247, t3719, t1222, t1261, t17232, t17237, t17243, t17244, t17247, t17250, t17254, t17258, t17260, t17261, t5384, t5386);
        let (t17278, t17280, t17283, t17288) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2227::<F>(t16750, t482, t371, t372, t1803, t3666, t1208, t5215);
        let t17289 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2228::<F>(t17288, t225);
        let (t17290, t17299) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2229::<F>(t17289, t480, t3678, t5327, t5323, t1235, t1238, t12800, t12976, t17280, t17283, t1791, t1808, t3644, t3663, t3667, t5320, t5391);
    (t17254, t17261, t17265, t17268, t17278, t17280, t17283, t17288, t17289, t17290, t17299)
}
