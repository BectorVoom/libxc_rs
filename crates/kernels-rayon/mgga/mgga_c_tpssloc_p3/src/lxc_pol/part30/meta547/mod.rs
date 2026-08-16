//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1898;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta547(t1433: f64, t1437: f64, t72: f64, t5445: f64, t79: f64, t5392: f64, t605: f64, t5399: f64, t1860: f64, t1865: f64, t22544: f64, t26013: f64, t26016: f64, t26051: f64, t26084: f64, t27937: f64, t27950: f64, t27953: f64, t27957: f64, t27961: f64, t27966: f64, t6490: f64, t7428: f64, t7432: f64, t7435: f64, t7442: f64, t7446: f64, t5: f64, t112: f64, t1868: f64, t5456: f64, t1873: f64, t19451: f64, t1441: f64, t1458: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27971, t27972, t27975, t27976, t27979, t27982, t27991) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1898(t1433, t1437, t72, t5445, t79, t5392, t605, t5399, t1860, t1865, t22544, t26013, t26016, t26051, t26084, t27937, t27950, t27953, t27957, t27961, t27966, t6490, t7428, t7432, t7435, t7442, t7446);
        let (t27992, t27993, t27996, t28001, t28002) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1899(t5, t27991, t112, t1868, t5456, t1873, t19451, t1441, t1458);
    (t27971, t27972, t27975, t27976, t27979, t27982, t27992, t27993, t27996, t28001, t28002)
}
