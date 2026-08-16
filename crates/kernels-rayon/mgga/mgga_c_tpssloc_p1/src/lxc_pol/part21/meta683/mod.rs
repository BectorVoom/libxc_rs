//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2495;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta683(t41189: f64, t4134: f64, t118: f64, t12971: f64, t2576: f64, t794: f64, t13025: f64, t9546: f64, t13017: f64, t2563: f64, t1489: f64, t41083: f64, t2559: f64, t4126: f64, t4130: f64, t12997: f64, t13000: f64, t2566: f64, t67: f64, t792: f64, t9558: f64, t12984: f64, t2379: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46772, t46780, t46782, t46788, t46790) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2495(t41189, t4134, t118, t12971, t2576, t794, t13025, t9546, t13017, t2563, t1489, t41083);
        let (t46793, t46796, t46799, t46802) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2496(t2559, t4126, t4130, t12997, t13000, t2566, t67, t792, t9558, t12984, t2379, t686);
    (t46772, t46780, t46782, t46788, t46790, t46793, t46796, t46799, t46802)
}
