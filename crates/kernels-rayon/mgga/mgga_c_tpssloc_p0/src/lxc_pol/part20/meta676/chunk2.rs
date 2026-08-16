//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2552/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2552(t449: f64, t51703: f64, t51723: f64, t11365: f64, t1694: f64, t3331: f64, t4794: f64, t1117: f64, t14913: f64, t3313: f64, t3315: f64, t11185: f64, t14937: f64) -> (f64, f64, f64, f64, f64) {
    let t51725 = (t51703 + t51723) * t449;
    let t51727 = t11365 * t1694;
    let t51730 = t4794 * t3331;
    let t51736 = 0.48245938496077605201e2_f64 * t3313 * t14913 * t3315 * t1117;
    let t51738 = 18.0_f64 * t11185 * t14937;
    (t51725, t51727, t51730, t51736, t51738)
}
