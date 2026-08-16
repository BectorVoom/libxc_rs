//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta995 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta995<F: Float>(t141: F, t2908: F, t63357: F, t11341: F, t63344: F, t41294: F, t63349: F, t2880: F, t63395: F, t41441: F, t63462: F, t63464: F, t63541: F, t63543: F, t63545: F, t63547: F, t63549: F, t63551: F, t63554: F, t63557: F, t63262: F, t63295: F, t63334: F, t63380: F, t63473: F, t63509: F, t63540: F, t915: F, t935: F, t41578: F, t6145: F, t11294: F, t19250: F, t15474: F, t2924: F, t4635: F, t63212: F, t63214: F, t63216: F, t63218: F, t63220: F, t63222: F, t63224: F, t63226: F, t63228: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t63560, t63563, t63566, t63568, t63573) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382::<F>(t141, t2908, t63357, t11341, t63344, t41294, t63349, t2880, t63395, t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557);
        let (t63579, t63581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383::<F>(t63262, t63295, t63334, t63380, t63473, t63509, t63540, t63573, t915, t935, t41578, t6145);
        let (t63583, t63586, t63587) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3384::<F>(t11294, t19250, t15474, t2924, t4635, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t63226, t63228, t63579, t63581);
    (t63560, t63563, t63566, t63568, t63579, t63581, t63583, t63586, t63587)
}
