//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta348(t59: f64, t598: f64, t535: f64, t795: f64, t215: f64, t39933: f64, t116: f64, t557: f64, t1314: f64, t9534: f64, t9223: f64, t120: f64, t212: f64, t22815: f64, t67: f64, t9580: f64, t2566: f64, t3732: f64, t12214: f64, t792: f64, t2229: f64, t60: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40344, t40347, t40350, t40353, t40369, t40394, t40399) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1141(t59, t598, t535, t795, t215, t39933, t116, t557, t1314, t9534, t9223, t120, t212, t22815, t67);
        let (t40401, t40406, t40409, t40412, t40419) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1142(t40394, t40399, t535, t1314, t9580, t2566, t3732, t12214, t792, t2229, t59, t60);
    (t40344, t40347, t40350, t40353, t40369, t40394, t40399, t40401, t40406, t40409, t40412, t40419)
}
