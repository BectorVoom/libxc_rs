//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1520;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1521;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1522;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta267(t849: f64, t9601: f64, t2697: f64, t2707: f64, t241: f64, t6589: f64, t67: f64, t2613: f64, t68: f64, t816: f64, t2632: f64, t776: f64, t2678: f64, t815: f64, t836: f64, t812: f64, t2649: f64, t2617: f64, t2642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9602, t9604, t9607, t9612) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1520(t849, t9601, t2697, t2707, t241, t6589, t67, t2613, t68);
        let (t9613, t9627, t9632) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1521(t816, t9612, t2632, t776, t2678);
        let (t9637, t9638) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1522(t815, t836, t812);
        let (t9639, t9642) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1523(t2649, t9638, t2617, t2642);
    (t9602, t9604, t9607, t9612, t9613, t9627, t9632, t9637, t9638, t9639, t9642)
}
