//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1177;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta376(t4211: f64, t9874: f64, t1472: f64, t9862: f64, t1519: f64, t9971: f64, t1496: f64, t41083: f64, t1516: f64, t40965: f64, t4166: f64, t9637: f64, t12985: f64, t9577: f64, t41189: f64, t4134: f64, t1489: f64, t133: f64, t1484: f64, t41214: f64, t6600: f64, t1512: f64, t41362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t46433, t46439, t46524, t46546, t46577, t46657) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1177(t4211, t9874, t1472, t9862, t1519, t9971, t1496, t41083, t1516, t40965, t4166, t9637);
        let (t46764, t46772, t46790, t46806, t46876) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178(t12985, t9577, t41189, t4134, t1489, t41083, t133, t1484, t41214, t6600, t1512, t41362);
    (t46433, t46439, t46524, t46546, t46577, t46657, t46764, t46772, t46790, t46806, t46876)
}
