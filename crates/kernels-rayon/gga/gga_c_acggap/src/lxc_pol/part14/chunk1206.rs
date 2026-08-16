//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1206/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1206(t615: f64, t6413: f64, t119: f64, t150: f64, t187: f64, t31905: f64, t33489: f64, t33511: f64, t33516: f64, t33518: f64, t33523: f64, t33525: f64, t33529: f64, t33533: f64, t33538: f64, t33541: f64, t33546: f64, t33554: f64, t40595: f64, t621: f64, t7931: f64, t8440: f64) -> f64 {
    let t40601 = t615 * t6413;
    let t40604 = -0.17347256376410398924e1_f64 * t33511 + t33516 - 0.17347256376410398924e1_f64 * t33518 + t33523 - 0.17347256376410398924e1_f64 * t33525 - t33529 - 0.17347256376410398924e1_f64 * t7931 * t33489 * t8440 + 0.65854491829355115987e0_f64 * t119 * t40595 * t150 * t187 + t33533 + t33538 - t33541 - 0.17347256376410398924e1_f64 * t31905 + t33546 - 0.4336814094102599731e0_f64 * t40601 * t621 + t33554;
    t40604
}
