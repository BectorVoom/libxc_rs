//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2499;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta685(t2570: f64, t2585: f64, t4255: f64, t46853: f64, t13326: f64, t9638: f64, t2628: f64, t2691: f64, t4184: f64, t812: f64, t1512: f64, t41362: f64, t4166: f64, t9666: f64, t2635: f64, t13337: f64, t838: f64, t2693: f64, t4163: f64, t41008: f64, t4155: f64, t13076: f64, t13322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46855, t46870, t46874, t46876) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2499(t2570, t2585, t4255, t46853, t13326, t9638, t2628, t2691, t4184, t812, t1512, t41362);
        let (t46882, t46884, t46886, t46911, t46918, t46920) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2500(t4166, t9666, t2635, t13337, t838, t2693, t4163, t41008, t4155, t13076, t9638, t13322);
    (t46855, t46870, t46874, t46876, t46882, t46884, t46886, t46911, t46918, t46920)
}
