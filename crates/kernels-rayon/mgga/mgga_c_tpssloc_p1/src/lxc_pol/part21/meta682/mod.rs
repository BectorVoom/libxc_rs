//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta682 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2493;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2494;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta682(t1509: f64, t2678: f64, t13225: f64, t9638: f64, t13312: f64, t41107: f64, t4240: f64, t13261: f64, t2617: f64, t812: f64, t836: f64, t9972: f64, t13265: f64, t13258: f64, t13333: f64, t12985: f64, t9577: f64, t212: f64, t4119: f64, t2586: f64, t9523: f64, t4138: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46693, t46698, t46717, t46733, t46737, t46741) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2493(t1509, t2678, t13225, t9638, t13312, t41107, t4240, t13261, t2617, t812, t836, t9972);
        let (t46742, t46748, t46764, t46766, t46768, t46770) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2494(t13265, t46741, t13258, t13333, t12985, t9577, t212, t4119, t2586, t9523, t4138, t9541);
    (t46693, t46698, t46717, t46733, t46737, t46741, t46742, t46748, t46764, t46766, t46768, t46770)
}
