//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta995 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta995(t141: f64, t2908: f64, t63357: f64, t11341: f64, t63344: f64, t41294: f64, t63349: f64, t2880: f64, t63395: f64, t41441: f64, t63462: f64, t63464: f64, t63541: f64, t63543: f64, t63545: f64, t63547: f64, t63549: f64, t63551: f64, t63554: f64, t63557: f64, t63262: f64, t63295: f64, t63334: f64, t63380: f64, t63473: f64, t63509: f64, t63540: f64, t915: f64, t935: f64, t41578: f64, t6145: f64, t11294: f64, t19250: f64, t15474: f64, t2924: f64, t4635: f64, t63212: f64, t63214: f64, t63216: f64, t63218: f64, t63220: f64, t63222: f64, t63224: f64, t63226: f64, t63228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63560, t63563, t63566, t63568, t63573) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3382(t141, t2908, t63357, t11341, t63344, t41294, t63349, t2880, t63395, t41441, t63462, t63464, t63541, t63543, t63545, t63547, t63549, t63551, t63554, t63557);
        let (t63579, t63581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383(t63262, t63295, t63334, t63380, t63473, t63509, t63540, t63573, t915, t935, t41578, t6145);
        let (t63583, t63586, t63587) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3384(t11294, t19250, t15474, t2924, t4635, t63212, t63214, t63216, t63218, t63220, t63222, t63224, t63226, t63228, t63579, t63581);
    (t63560, t63563, t63566, t63568, t63579, t63581, t63583, t63586, t63587)
}
