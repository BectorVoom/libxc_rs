//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 931/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk931(t20264: f64, t33: f64, t20217: f64, t20234: f64, t4007: f64, t4012: f64, t5398: f64, t634: f64, t638: f64, t9321: f64, t9330: f64, t72: f64) -> (f64, f64) {
    let t20265 = t33 * t20264;
    let t20284 = -280.0_f64 / 27.0_f64 * t9321 * t20234 + 28.0_f64 / 3.0_f64 * t4007 * t5398 - 4.0_f64 / 3.0_f64 * t634 * t20217 + 280.0_f64 / 27.0_f64 * t9330 * t20234 + 28.0_f64 / 3.0_f64 * t4012 * t5398 + 4.0_f64 / 3.0_f64 * t638 * t20217;
    let t20285 = t72 * t20284;
    (t20265, t20285)
}
