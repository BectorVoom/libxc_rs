//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2105;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta590<F: Float>(t41189: F, t4134: F, t13025: F, t9546: F, t1489: F, t41083: F, t2559: F, t4126: F, t4130: F, t12997: F, t13000: F, t2566: F, t67: F, t792: F, t9558: F, t133: F, t1484: F, t41214: F, t6600: F, t213: F, t221: F, t13004: F, t782: F, t131: F, t205: F, t41160: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t46772, t46783, t46790, t46794, t46796) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2104::<F>(t41189, t4134, t13025, t9546, t1489, t41083, t2559, t4126, t4130, t12997, t13000, t2566);
        let (t46799, t46806, t46838, t46843, t46847) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2105::<F>(t67, t792, t9558, t133, t1484, t41214, t6600, t213, t221, t13004, t782, t131, t205, t41160);
    (t46772, t46783, t46790, t46794, t46796, t46799, t46806, t46838, t46843, t46847)
}
