//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta693 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta693(t25: f64, t53796: f64, t5154: f64, t9919: f64, t39305: f64, t3665: f64, t584: f64, t2249: f64, t606: f64, t16: f64, t5173: f64, t591: f64, t11987: f64, t11988: f64, t1298: f64, t1408: f64, t15989: f64, t15992: f64, t2: f64, t3704: f64, t39861: f64, t5170: f64, t9257: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2642(t25, t53796, t5154, t9919, t39305, t3665, t584, t2249, t606, t16, t5173, t591, t11987, t11988, t1298, t1408, t15989, t15992, t2, t3704, t39861, t5170, t9257, zeta_threshold);
    (t53797, t53799, t53800, t53805, t53808, t53814, t53817, t53827)
}
