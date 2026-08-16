//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1686;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta318(t225: f64, t3591: f64, t3482: f64, t3639: f64, t500: f64, t3696: f64, t588: f64, t592: f64, t1285: f64, t2223: f64, t1287: f64, t1291: f64, t9874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11925, t11928, t11947, t11975, t11977, t11979, t11981, t11984) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1686(t225, t3591, t3482, t3639, t500, t3696, t588, t592, t1285, t2223, t1287, t1291, t9874);
    (t11925, t11928, t11947, t11975, t11977, t11979, t11981, t11984)
}
