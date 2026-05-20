//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta792 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2857;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2858;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2859;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2860;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2861;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2862;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2863;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2864;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2865;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2866;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta792<F: Float>(t52013: F, t52016: F, t52020: F, t52023: F, t52025: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t15155: F, t689: F, t15131: F, t15136: F, t15141: F, t128: F, t2850: F, t51905: F, t51925: F, t11142: F, t51930: F, t52002: F, t904: F, t15199: F, t698: F, t141: F, t51969: F, t930: F, t51973: F, t41329: F, t41361: F, t41363: F, t41369: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51961: F, t51965: F, t51967: F, t51971: F, t51978: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t52043 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2857::<F>(t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041);
        let t52045 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2858::<F>(t15155, t689);
        let t52047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2859::<F>(t15131, t689);
        let t52049 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2860::<F>(t15136, t689);
        let t52051 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2861::<F>(t15141, t689);
        let t52054 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2862::<F>(t128, t2850, t51905);
        let t52057 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2863::<F>(t128, t2850, t51925);
        let t52060 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2864::<F>(t11142, t128, t51930);
        let t52063 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2865::<F>(t128, t52002, t904);
        let (t52065, t52068, t52090) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2866::<F>(t15199, t698, t141, t51969, t930, t51973, t41329, t41361, t41363, t41369, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
    (t52043, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068, t52090)
}
