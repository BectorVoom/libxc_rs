//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1033/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1033(t1985: f64, t28232: f64, t31611: f64, t115545: f64, t22633: f64, t28116: f64, t113941: f64, t115331: f64, t122133: f64, t127187: f64, t127197: f64, t127201: f64, t127202: f64, t20029: f64, t20060: f64, t2092: f64, t26224: f64, t26989: f64, t28219: f64, t31653: f64, t33294: f64, t33301: f64, t33316: f64, t5215: f64, t6440: f64, t8627: f64, t96913: f64) -> f64 {
    let t128656 = t1985 * t31611 * t28232;
    let t128659 = t22633 * t115545 * t28116;
    let t128663 = -t113941 - t96913 * t2092 + 0.38381794893125283518e-1_f64 * t122133 - t127187 - 2.0_f64 * t5215 * t33294 - t115331 + 2.0_f64 * t20060 * t8627 + 4.0_f64 * t20029 * t8627 + 2.0_f64 * t31653 * t6440 - 12.0_f64 * t26224 * t26989 * t28219 + 4.0_f64 * t5215 * t33301 - t127197 + 0.16449340668482264365e-1_f64 * t128656 - t127201 - t127202 + 0.3289868133696452873e-1_f64 * t128659 + 4.0_f64 * t5215 * t33316;
    t128663
}
