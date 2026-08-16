//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1552;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1553;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1554;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta276(t2374: f64, t9882: f64, t2535: f64, t677: f64, t2528: f64, t2509: f64, t745: f64, t9843: f64, t761: f64, t152: f64, t31: f64, t2448: f64, t67: f64, t758: f64, t2368: f64, t2505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9884, t9885) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1552(t2374, t9882, t2535, t677);
        let (t9887, t9888) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1553(t2374, t9885, t2528, t677);
        let (t9890, t9892) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1554(t2374, t9888, t2509, t745, t9843);
        let (t9894, t9897, t9901, t9902, t9905) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1555(t761, t9892, t152, t31, t2448, t67, t758, t2368, t2505, t745);
    (t9884, t9885, t9887, t9888, t9890, t9892, t9894, t9897, t9901, t9902, t9905)
}
