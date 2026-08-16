//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta800 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2903;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2904;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2905;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta800(t3011: f64, t4682: f64, t11506: f64, t1626: f64, t1609: f64, t2924: f64, t11112: f64, t2875: f64, t4632: f64, t11294: f64, t15098: f64, t51849: f64, t51853: f64, t51858: f64, t51863: f64, t51867: f64, t51871: f64, t51875: f64, t51878: f64, t51881: f64, t51884: f64, t51887: f64, t51890: f64, t51892: f64, t51894: f64, t51896: f64, t51899: f64, t51902: f64, t51907: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t41267: f64, t41275: f64, t41592: f64, t51921: f64, t51923: f64, t51927: f64, t51932: f64, t51935: f64, t51937: f64, t51940: f64, t51942: f64, t51945: f64, t51973: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64, t41292: f64, t41610: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64, t41361: f64, t41363: f64, t41369: f64, t51978: f64, t51981: f64, t51984: f64, t51987: f64, t51990: f64, t51995: f64, t52000: f64, t52004: f64, t52013: f64, t52016: f64, t52020: f64, t52023: f64, t52025: f64, t52028: f64, t52031: f64, t52033: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t41406: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52065: f64, t52068: f64, t52116: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52637, t52642, t52647, t52650, t52652, t52664) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901(t3011, t4682, t11506, t1626, t1609, t2924, t11112, t2875, t4632, t11294, t15098, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51878, t51881, t51884, t51887);
        let t52677 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902(t51890, t51892, t51894, t51896, t51899, t51902, t51907, t51909, t51911, t51913, t51915, t51917);
        let t52690 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2903(t41267, t41275, t41592, t51921, t51923, t51927, t51932, t51935, t51937, t51940, t51942, t51945);
        let t52702 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2904(t51973, t41281, t41283, t41285, t41287, t41289, t41292, t41610, t51961, t51965, t51967, t51971);
        let (t52716, t52729) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2905(t41361, t41363, t41369, t51978, t51981, t51984, t51987, t51990, t51995, t52000, t52004, t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041);
        let t52743 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2906(t41406, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068, t52116);
    (t52637, t52642, t52647, t52650, t52652, t52664, t52677, t52690, t52702, t52716, t52729, t52743)
}
