//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta171 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk804;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta171(t2649: f64, t9638: f64, t2617: f64, t2642: f64, t1891: f64, t67: f64, t246: f64, t232: f64, t2379: f64, t2646: f64, t2645: f64, t2647: f64, t9626: f64, t210: f64, t2553: f64, t804: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9639, t9642) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk804(t2649, t9638, t2617, t2642);
        let (t9645, t9646, t9647, t9649, t9653, t9657) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk805(t1891, t67, t246, t232, t2379, t2646, t2645, t2647, t9626, t210, t2553, t804);
    (t9639, t9642, t9645, t9646, t9647, t9649, t9653, t9657)
}
