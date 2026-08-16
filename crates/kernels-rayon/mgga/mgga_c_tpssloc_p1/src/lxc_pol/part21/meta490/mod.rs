//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta490 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2096;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta490(t828: f64, t9975: f64, t16815: f64, t16758: f64, t4182: f64, t2732: f64, t5617: f64, t829: f64, t1499: f64, t4290: f64, t4166: f64, t4177: f64, t120: f64, t5584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2096(t828, t9975, t16815, t16758, t4182, t2732, t5617, t829, t1499, t4290, t4166, t4177);
        let t16839 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2097(t120, t5584);
    (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836, t16839)
}
