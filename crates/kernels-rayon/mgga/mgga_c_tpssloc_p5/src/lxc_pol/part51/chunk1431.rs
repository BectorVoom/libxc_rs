//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1431/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1431(t22674: f64, t33296: f64, t6897: f64, t22751: f64, t33307: f64, t114178: f64, t115530: f64, t115540: f64, t115551: f64, t115619: f64, t120327: f64, t120334: f64, t120337: f64, t1843: f64, t22656: f64, t26477: f64, t31555: f64, t31655: f64, t5215: f64, t7199: f64, t7937: f64, t97740: f64) -> f64 {
    let t122247 = t6897 * t22674 * t33296;
    let t122251 = t22751 * t33307;
    let t122255 = -0.19190897446562641759e-1_f64 * t115530 - t114178 + 2.0_f64 * t5215 * t31555 + 2.0_f64 * t26477 * t7199 - t115540 + 0.41123351671205660912e-2_f64 * t122247 - 6.0_f64 * t97740 * t31655 + t120327 + 0.38381794893125283518e-1_f64 * t122251 + t115551 - t22656 * t7937 - t115619 * t1843 + t120334 - t120337;
    t122255
}
