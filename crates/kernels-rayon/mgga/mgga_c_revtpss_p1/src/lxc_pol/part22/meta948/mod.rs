//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta948 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3187;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta948(t12809: f64, t12916: f64, t17483: f64, t12772: f64, t17729: f64, t17731: f64, t3718: f64, t44546: f64, t5353: f64, t45833: f64, t58919: f64, t127: f64, t17693: f64, t17695: f64, t5302: f64, t1261: f64, t12879: f64, t247: f64, t5056: f64, t12963: f64, t5323: f64, t225: f64, t56587: f64, t17795: f64, t3172: f64, t3711: f64, t17759: f64, t44425: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59179, t59182, t59185, t59196, t59220) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3187(t12809, t12916, t17483, t12772, t17729, t17731, t3718, t44546, t5353, t45833, t58919, t127, t17693, t17695, t5302);
        let (t59233, t59239, t59241, t59269, t59320) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3188(t1261, t12879, t247, t5056, t12963, t5323, t225, t56587, t17795, t3172, t3711, t17729, t17759, t44425);
    (t59179, t59182, t59185, t59196, t59220, t59233, t59239, t59241, t59269, t59320)
}
