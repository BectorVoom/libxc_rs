//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1930;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta415(t14841: f64, t3404: f64, t1164: f64, t1098: f64, t4737: f64, t1119: f64, t3308: f64, t4740: f64, t1657: f64, t3312: f64, t3316: f64, t11282: f64, t1694: f64, t11285: f64, t3377: f64, t300: f64, t4832: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14842, t14844, t14845, t14847, t14849, t14850) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1930(t14841, t3404, t1164, t1098, t4737, t1119, t3308, t4740, t1657, t3312);
        let (t14852, t14854, t14855, t14857, t14858) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1931(t14850, t3316, t11282, t1694, t11285, t3377, t1164, t300, t4832);
    (t14842, t14844, t14845, t14847, t14849, t14850, t14852, t14854, t14855, t14857, t14858)
}
