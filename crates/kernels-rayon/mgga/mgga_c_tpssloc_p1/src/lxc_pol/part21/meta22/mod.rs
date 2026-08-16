//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk170;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk171;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk172;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk173;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta22(t407: f64, t410: f64, t413: f64, t417: f64, t409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t419, t422, t423) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk170(t407, t410, t413, t417);
        let (t425, t427) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk171(t409, t423, t407);
        let (t432, t435, t436) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk172(t407, t410, t413, t417);
        let t440 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk173(t407);
        let (t445, t448, t449) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk174(t407, t410, t413, t417);
    (t419, t422, t423, t425, t427, t432, t435, t436, t440, t445, t448, t449)
}
