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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta792(t52013: f64, t52016: f64, t52020: f64, t52023: f64, t52025: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t15155: f64, t689: f64, t15131: f64, t15136: f64, t15141: f64, t128: f64, t2850: f64, t51905: f64, t51925: f64, t11142: f64, t51930: f64, t52002: f64, t904: f64, t15199: f64, t698: f64, t141: f64, t51969: f64, t930: f64, t51973: f64, t41329: f64, t41361: f64, t41363: f64, t41369: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t51978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t52043 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2857(t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041);
        let t52045 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2858(t15155, t689);
        let t52047 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2859(t15131, t689);
        let t52049 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2860(t15136, t689);
        let t52051 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2861(t15141, t689);
        let t52054 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2862(t128, t2850, t51905);
        let t52057 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2863(t128, t2850, t51925);
        let t52060 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2864(t11142, t128, t51930);
        let t52063 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2865(t128, t52002, t904);
        let (t52065, t52068, t52090) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2866(t15199, t698, t141, t51969, t930, t51973, t41329, t41361, t41363, t41369, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51961, t51965, t51967, t51971, t51978, t52028, t52031, t52033);
    (t52043, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068, t52090)
}
