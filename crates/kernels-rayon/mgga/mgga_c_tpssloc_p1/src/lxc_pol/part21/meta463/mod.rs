//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2032;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2033;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta463(t16081: f64, t5198: f64, t213: f64, t5187: f64, t1307: f64, t221: f64, t3719: f64, t5196: f64, t3732: f64, t67: f64, t792: f64, t1799: f64, t212: f64, t686: f64, t12214: f64, t131: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16083, t16086, t16090, t16093, t16094) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2032(t16081, t5198, t213, t5187, t1307, t221, t3719, t5196, t3732, t67, t792);
        let t16095 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2033(t1799, t212);
        let (t16097, t16099, t16100, t16101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2034(t1307, t16095, t686, t16094, t12214, t131, t205);
    (t16083, t16086, t16090, t16093, t16094, t16095, t16097, t16099, t16100, t16101)
}
