//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1470;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1471;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1472;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1473;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta281(t10647: f64, t291: f64, t2784: f64, t892: f64, t914: f64, t2787: f64, t2837: f64, t2841: f64, t888: f64, t2845: f64, t10521: f64, t10528: f64, t10607: f64, t10622: f64, t10625: f64, t10627: f64, t10635: f64, t2840: f64, t287: f64, t275: f64, t2793: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10649, t10650, t10652, t10654, t10655) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1470(t10647, t291, t2784, t892, t914, t2787, t2837, t2841, t888);
        let (t10657, t10658) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1471(t10655, t2845, t10521, t10528, t10607, t10622, t10625, t10627, t10635, t10649, t10652, t10654);
        let (t10660, t10661) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1472(t2840, t287, t275);
        let t10662 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1473(t2793, t912);
    (t10649, t10650, t10652, t10654, t10655, t10657, t10658, t10660, t10661, t10662)
}
