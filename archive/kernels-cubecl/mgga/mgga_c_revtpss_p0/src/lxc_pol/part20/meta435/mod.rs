//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1639;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1641;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta435<F: Float>(t1224: F, t12268: F, t1222: F, t3688: F, t697: F, t13001: F, t140: F, t1226: F, t2438: F, t12855: F, t12857: F, t12916: F, t1012: F, t13095: F, t17261: F, t3699: F, t39443: F, t39449: F, t43847: F, t43852: F, t44898: F, t44902: F, t44906: F, t44912: F, t44917: F, t5308: F, t5312: F, t1214: F, t12621: F, t12956: F, t12959: F, t3566: F, t3781: F, t5330: F, t3362: F, t404: F, t43766: F, t13007: F, t13028: F, t13026: F, t43776: F, t3700: F, t43750: F, t43757: F, t43759: F, t43761: F, t43965: F, t43970: F, t43980: F, t43982: F, t44011: F, t44014: F, t44021: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44919, t44925, t44928, t44931, t44938) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1639::<F>(t1224, t12268, t1222, t3688, t697, t13001, t140, t1226, t2438, t12855, t12857, t12916);
        let t44942 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640::<F>(t1012, t1222, t13095, t17261, t3699, t39443, t39449, t43847, t43852, t44898, t44902, t44906, t44912, t44917, t44919, t44925, t44928, t44931, t44938, t5308, t5312);
        let (t44944, t44949, t44952, t44959, t44965) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1641::<F>(t1214, t12621, t12956, t12959, t3566, t3781, t5330, t3362, t404, t43766, t1222, t13007, t140);
        let (t44972, t44974, t44980, t44982) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642::<F>(t1222, t13028, t140, t13026, t43776, t3700, t697, t43750, t43757, t43759, t43761, t43965, t43970, t43980, t43982, t44011, t44014, t44021);
    (t44942, t44944, t44949, t44952, t44959, t44965, t44972, t44974, t44980, t44982)
}
