//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1556;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta277(t761: f64, t9905: f64, t2250: f64, t751: f64, t707: f64, t2447: f64, t706: f64, t2509: f64, t746: f64, t9490: f64, t2531: f64, t2535: f64, t2427: f64, t2430: f64, t32: f64, t717: f64, t2244: f64, t2658: f64, t2617: f64, t2629: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9907, t9909, t9910, t9912, t9919) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1556(t761, t9905, t2250, t751, t707, t2447, t706, t2509, t746, t9490);
        let (t9921, t9922, t9924, t9929, t9932, t9933, t9967) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1557(t761, t9919, t2531, t2535, t2427, t2430, t32, t717, t2244, t751, t2658, t2617, t2629);
    (t9907, t9909, t9910, t9912, t9919, t9921, t9922, t9924, t9929, t9932, t9933, t9967)
}
