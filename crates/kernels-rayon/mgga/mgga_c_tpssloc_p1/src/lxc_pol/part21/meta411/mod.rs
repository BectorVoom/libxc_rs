//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1920;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1921;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1922;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta411(t1088: f64, t14753: f64, t123: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11247: f64, t14702: f64, t14708: f64, t14721: f64, t14723: f64, t14724: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t1100: f64, t1667: f64, t2403: f64, t14720: f64, t11215: f64, t11217: f64, t14722: f64, t11219: f64, t14726: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14754, t14755) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1920(t1088, t14753, t123);
        let t14758 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1921(t11137, t11139, t11141, t11143, t11247, t14702, t14708, t14721, t14723, t14724, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14759, t14766) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1922(t1100, t14758, t1667, t2403);
        let (t14768, t14776, t14778) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1923(t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755, t14766, t11219, t14726);
    (t14754, t14755, t14758, t14759, t14766, t14768, t14776, t14778)
}
