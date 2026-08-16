//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2356;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2357;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2358;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2359;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta602(t39300: f64, t739: f64, t746: f64, t1294: f64, t3691: f64, t9722: f64, t2483: f64, t268: f64, t9778: f64, t2406: f64, t9790: f64, t204: f64, t2410: f64, t2415: f64, t676: f64, t9452: f64, t9455: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t39302, t39304, t39305, t39309) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2356(t39300, t739, t746, t1294, t3691, t9722, t2483, t268, t9778);
        let t39312 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2357(t2406, t268, t9790);
        let t39316 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2358(t204, t2410, t2415, t268);
        let t39320 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2359(t268, t676, t9452, t9455);
    (t39302, t39304, t39305, t39309, t39312, t39316, t39320)
}
