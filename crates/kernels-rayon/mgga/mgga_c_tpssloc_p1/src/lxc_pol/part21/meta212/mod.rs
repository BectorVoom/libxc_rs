//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta212 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1298;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1299;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1300;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1301;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta212(t25: f64, t28: f64, t17: f64, t5168: f64, t1408: f64, t3704: f64, t1298: f64, t2: f64, t584: f64, t606: f64, t1649: f64, t3711: f64, t1302: f64, t1081: f64, zeta_threshold: f64, t1804: f64, t3726: f64, t131: f64, t3732: f64, t205: f64, t1799: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5169, t5170, t5173, t5177, t5178, t5181, t5185) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1298(t25, t28, t17, t5168, t1408, t3704, t1298, t2, t584, t606, t1649, t3711, t1302, t1081, zeta_threshold);
        let t5187 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1299(t5177, t5185);
        let (t5192, t5194, t5195) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1300(t1804, t3726, t131, t3732, t205);
        let t5196 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1301(t1799, t213);
    (t5169, t5170, t5173, t5178, t5181, t5187, t5192, t5194, t5195, t5196)
}
