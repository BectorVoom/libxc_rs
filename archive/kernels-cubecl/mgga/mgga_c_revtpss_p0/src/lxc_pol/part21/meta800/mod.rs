//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta800 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2903;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2904;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2905;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta800<F: Float>(t3011: F, t4682: F, t11506: F, t1626: F, t1609: F, t2924: F, t11112: F, t2875: F, t4632: F, t11294: F, t15098: F, t51849: F, t51853: F, t51858: F, t51863: F, t51867: F, t51871: F, t51875: F, t51878: F, t51881: F, t51884: F, t51887: F, t51890: F, t51892: F, t51894: F, t51896: F, t51899: F, t51902: F, t51907: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t41267: F, t41275: F, t41592: F, t51921: F, t51923: F, t51927: F, t51932: F, t51935: F, t51937: F, t51940: F, t51942: F, t51945: F, t51973: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41292: F, t41610: F, t51961: F, t51965: F, t51967: F, t51971: F, t41361: F, t41363: F, t41369: F, t51978: F, t51981: F, t51984: F, t51987: F, t51990: F, t51995: F, t52000: F, t52004: F, t52013: F, t52016: F, t52020: F, t52023: F, t52025: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F, t41406: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52065: F, t52068: F, t52116: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52637, t52642, t52647, t52650, t52652, t52664) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2901::<F>(t3011, t4682, t11506, t1626, t1609, t2924, t11112, t2875, t4632, t11294, t15098, t51849, t51853, t51858, t51863, t51867, t51871, t51875, t51878, t51881, t51884, t51887);
        let t52677 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2902::<F>(t51890, t51892, t51894, t51896, t51899, t51902, t51907, t51909, t51911, t51913, t51915, t51917);
        let t52690 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2903::<F>(t41267, t41275, t41592, t51921, t51923, t51927, t51932, t51935, t51937, t51940, t51942, t51945);
        let t52702 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2904::<F>(t51973, t41281, t41283, t41285, t41287, t41289, t41292, t41610, t51961, t51965, t51967, t51971);
        let (t52716, t52729) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2905::<F>(t41361, t41363, t41369, t51978, t51981, t51984, t51987, t51990, t51995, t52000, t52004, t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041);
        let t52743 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2906::<F>(t41406, t52045, t52047, t52049, t52051, t52054, t52057, t52060, t52063, t52065, t52068, t52116);
    (t52637, t52642, t52647, t52650, t52652, t52664, t52677, t52690, t52702, t52716, t52729, t52743)
}
