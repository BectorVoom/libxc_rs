//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2541;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta708(t2841: f64, t4351: f64, t10701: f64, t1543: f64, t10810: f64, t1561: f64, t14363: f64, t942: f64, t2929: f64, t4446: f64, t1568: f64, t2886: f64, t2860: f64, t4408: f64, t10770: f64, t10811: f64, t14255: f64, t892: f64, t2791: f64, t10660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49269, t49274, t49285, t49404, t49411, t49422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2541(t2841, t4351, t10701, t1543, t10810, t1561, t14363, t942, t2929, t4446, t1568, t2886);
        let (t49427, t49430, t49478, t49483, t49486, t49489) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2542(t2860, t4408, t10770, t1561, t10811, t1568, t14255, t892, t2791, t4351, t10660, t1543);
    (t49269, t49274, t49285, t49404, t49411, t49422, t49427, t49430, t49478, t49483, t49486, t49489)
}
