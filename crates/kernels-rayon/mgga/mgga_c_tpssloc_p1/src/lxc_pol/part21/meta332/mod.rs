//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1708;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta332(t3734: f64, t550: f64, t3777: f64, t3802: f64, t225: f64, t3755: f64, t3700: f64, t570: f64, t1390: f64, t3914: f64, t3719: f64, t571: f64, t3698: f64, t3701: f64, t112: f64, t3931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12420, t12429) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1708(t3734, t550, t3777, t3802);
        let (t12444, t12461, t12466, t12470, t12477, t12521) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1709(t225, t3755, t3700, t570, t1390, t3914, t3719, t571, t3698, t3701, t112, t3931);
    (t12420, t12429, t12444, t12461, t12466, t12470, t12477, t12521)
}
