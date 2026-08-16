//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta201 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1272;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1273;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1274;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1275;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1276;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1277;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1278;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1279;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1280;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta201<F: Float>(t1066: F, t4583: F, t247: F, t1062: F, t1659: F, t3204: F, t3116: F, t4757: F, t127: F, t1663: F, t371: F, t1025: F, t1063: F, t1068: F, t1675: F, t3106: F, t3112: F, t3127: F, t3174: F, t3188: F, t4818: F, t4821: F, t4825: F, t373: F, t4772: F, t372: F, t225: F, t4746: F, t366: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4716: F, t4718: F, t4721: F, t4723: F, t4727: F, t4731: F, t4736: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t4831 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1272::<F>(t1066, t4583, t247);
        let t4834 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1273::<F>(t1062, t1659);
        let t4837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1274::<F>(t1062, t3204);
        let t4839 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1275::<F>(t3116, t4757, t247);
        let t4845 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1276::<F>(t127, t1663, t371);
        let (t4846, t4848) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1277::<F>(t1025, t4845, t1063, t1068, t1675, t3106, t3112, t3127, t3174, t3188, t4818, t4821, t4825, t4831, t4834, t4837, t4839);
        let (t4852, t4854) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1278::<F>(t373, t4772, t371, t372);
        let t4857 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1279::<F>(t225, t4746);
        let t4858 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1280::<F>(t366, t4857);
        let t4866 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1281::<F>(t4589, t4592, t4594, t4597, t4634, t4638, t4716, t4718, t4721, t4723, t4727, t4731, t4736);
    (t4831, t4834, t4837, t4839, t4845, t4846, t4848, t4852, t4854, t4857, t4858, t4866)
}
