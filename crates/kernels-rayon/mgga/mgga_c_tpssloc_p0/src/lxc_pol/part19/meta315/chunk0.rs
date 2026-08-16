//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1121/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121(t28: f64, t3673: f64, t3231: f64, t39109: f64, t11122: f64, t12072: f64, t12075: f64, t3672: f64, t39436: f64, t517: f64, t157: f64, t39434: f64, t182: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t39437 = t3673 * t3673;
    let t39443 = t3231 * t3231;
    let t39448 = -t39109;
    let t39452 = piecewise3(t29, 0.0_f64, 40.0_f64 / 81.0_f64 * t39436 * t39437 - 16.0_f64 / 9.0_f64 * t12072 * t3673 * t3231 + 4.0_f64 / 3.0_f64 * t3672 * t39443 + 16.0_f64 / 9.0_f64 * t12075 * t11122 + 4.0_f64 / 3.0_f64 * t517 * t39448);
    let t39454 = (t39434 + t39452) * t157;
    let t39456 = 0.19751673498613801407e-1_f64 * t39454 * t182;
    (t39437, t39443, t39448, t39454, t39456)
}
