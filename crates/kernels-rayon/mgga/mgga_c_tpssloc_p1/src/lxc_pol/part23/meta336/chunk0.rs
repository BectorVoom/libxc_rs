//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1106/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1106(t39300: f64, t739: f64, t746: f64, t1294: f64, t2483: f64, t268: f64, t9778: f64) -> (f64, f64, f64) {
    let t39302 = t739 * t39300 * t746;
    let t39304 = 0.5848223622634646207e0_f64 * t1294 * t39302;
    let t39309 = 0.71233333333333333332e-1_f64 * t268 * t2483 * t9778;
    (t39302, t39304, t39309)
}
