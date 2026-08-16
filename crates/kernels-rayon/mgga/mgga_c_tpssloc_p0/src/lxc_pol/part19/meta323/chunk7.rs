//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1150/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1150(t28: f64, t11122: f64, t12000: f64, t12004: f64, t1302: f64, t3231: f64, t3673: f64, t3711: f64, t39437: f64, t39443: f64, t39448: f64, t39877: f64, t39874: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t39890 = piecewise3(t29, 0.0_f64, -56.0_f64 / 81.0_f64 * t39877 * t39437 + 16.0_f64 / 9.0_f64 * t12000 * t3673 * t3231 - 2.0_f64 / 3.0_f64 * t3711 * t39443 - 8.0_f64 / 9.0_f64 * t12004 * t11122 + 2.0_f64 / 3.0_f64 * t1302 * t39448);
    let t39892 = t39874 / 2.0_f64 + t39890 / 2.0_f64;
    t39892
}
