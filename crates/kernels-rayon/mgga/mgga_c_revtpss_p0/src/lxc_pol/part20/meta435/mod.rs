//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1639;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1641;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta435(t1224: f64, t12268: f64, t1222: f64, t3688: f64, t697: f64, t13001: f64, t140: f64, t1226: f64, t2438: f64, t12855: f64, t12857: f64, t12916: f64, t1012: f64, t13095: f64, t17261: f64, t3699: f64, t39443: f64, t39449: f64, t43847: f64, t43852: f64, t44898: f64, t44902: f64, t44906: f64, t44912: f64, t44917: f64, t5308: f64, t5312: f64, t1214: f64, t12621: f64, t12956: f64, t12959: f64, t3566: f64, t3781: f64, t5330: f64, t3362: f64, t404: f64, t43766: f64, t13007: f64, t13028: f64, t13026: f64, t43776: f64, t3700: f64, t43750: f64, t43757: f64, t43759: f64, t43761: f64, t43965: f64, t43970: f64, t43980: f64, t43982: f64, t44011: f64, t44014: f64, t44021: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44919, t44925, t44928, t44931, t44938) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1639(t1224, t12268, t1222, t3688, t697, t13001, t140, t1226, t2438, t12855, t12857, t12916);
        let t44942 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640(t1012, t1222, t13095, t17261, t3699, t39443, t39449, t43847, t43852, t44898, t44902, t44906, t44912, t44917, t44919, t44925, t44928, t44931, t44938, t5308, t5312);
        let (t44944, t44949, t44952, t44959, t44965) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1641(t1214, t12621, t12956, t12959, t3566, t3781, t5330, t3362, t404, t43766, t1222, t13007, t140);
        let (t44972, t44974, t44980, t44982) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1642(t1222, t13028, t140, t13026, t43776, t3700, t697, t43750, t43757, t43759, t43761, t43965, t43970, t43980, t43982, t44011, t44014, t44021);
    (t44942, t44944, t44949, t44952, t44959, t44965, t44972, t44974, t44980, t44982)
}
