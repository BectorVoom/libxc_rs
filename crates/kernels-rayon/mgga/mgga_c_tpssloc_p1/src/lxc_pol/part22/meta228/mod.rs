//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta228 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1290;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1291;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta228(t2588: f64, t9577: f64, t21: f64, t59: f64, t207: f64, t795: f64, t2690: f64, t841: f64, t812: f64, t849: f64, t241: f64, t6589: f64, t67: f64, t2632: f64, t776: f64, t815: f64, t836: f64, t2617: f64, t2642: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9579, t9580, t9583, t9600, t9601) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1289(t2588, t9577, t21, t59, t207, t795, t2690, t841, t812);
        let (t9602, t9607, t9627, t9637, t9638) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1290(t849, t9601, t241, t6589, t67, t2632, t776, t815, t836, t812);
        let t9642 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1291(t2617, t2642);
    (t9579, t9580, t9583, t9600, t9601, t9602, t9607, t9627, t9637, t9638, t9642)
}
