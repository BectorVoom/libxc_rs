//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1149/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1149(t25: f64, t11987: f64, t11991: f64, t1298: f64, t2249: f64, t3665: f64, t3704: f64, t39109: f64, t39420: f64, t39426: f64, t39861: f64, t9257: f64, t11998: f64, t28: f64, t517: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t39874 = piecewise3(t26, 0.0_f64, -56.0_f64 / 81.0_f64 * t39861 * t39420 + 16.0_f64 / 9.0_f64 * t11987 * t3665 * t2249 - 2.0_f64 / 3.0_f64 * t3704 * t39426 - 8.0_f64 / 9.0_f64 * t11991 * t9257 + 2.0_f64 / 3.0_f64 * t1298 * t39109);
    let t39877 = 1.0_f64 / t517 / t11998 / t28;
    (t39874, t39877)
}
