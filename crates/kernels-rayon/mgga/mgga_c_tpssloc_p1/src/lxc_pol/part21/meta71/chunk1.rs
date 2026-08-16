//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 520/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk520(t1409: f64, t634: f64, t638: f64, t72: f64) -> (f64, f64, f64) {
    let t1430 = t634 * t1409;
    let t1431 = t638 * t1409;
    let t1433 = -4.0_f64 / 3.0_f64 * t1430 + 4.0_f64 / 3.0_f64 * t1431;
    let t1434 = t72 * t1433;
    (t1430, t1431, t1434)
}
