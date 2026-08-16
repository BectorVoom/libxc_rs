//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1120/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1120(t25: f64, t11985: f64, t526: f64, t3665: f64, t2249: f64, t12061: f64, t12064: f64, t3664: f64, t39109: f64, t514: f64, t9257: f64, t11998: f64, t528: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t39419 = 1.0_f64 / t526 / t11985;
    let t39420 = t3665 * t3665;
    let t39426 = t2249 * t2249;
    let t39434 = piecewise3(t26, 0.0_f64, 40.0_f64 / 81.0_f64 * t39419 * t39420 - 16.0_f64 / 9.0_f64 * t12061 * t3665 * t2249 + 4.0_f64 / 3.0_f64 * t3664 * t39426 + 16.0_f64 / 9.0_f64 * t12064 * t9257 + 4.0_f64 / 3.0_f64 * t514 * t39109);
    let t39436 = 1.0_f64 / t528 / t11998;
    (t39420, t39426, t39434, t39436)
}
