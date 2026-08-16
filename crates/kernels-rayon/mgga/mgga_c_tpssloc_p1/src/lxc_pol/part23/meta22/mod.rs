//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk167;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk168;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk169;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk170;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk171;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta22(t407: f64, t410: f64, t413: f64, t417: f64, t300: f64, t425: f64, t427: f64, t338: f64, t51: f64, t405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t432, t435, t436) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk167(t407, t410, t413, t417);
        let t440 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk168(t407);
        let (t445, t448, t449) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk169(t407, t410, t413, t417);
        let (t453, t455, t456) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk170(t440, t449, t300, t425, t427, t436, t338, t51);
        let t457 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk171(t405);
    (t432, t435, t436, t440, t445, t448, t449, t453, t455, t456, t457)
}
