//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1713/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1713(t109: f64, t22468: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64, t2332: f64, t2358: f64, t6530: f64) -> (f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t22469 = 11.0_f64 / 9.0_f64 * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22472 = 2.0_f64 / 3.0_f64 * t22471;
    let t22473 = t63 * t2331;
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    let t22479 = piecewise3(t110, 0.0_f64, t22469 + t22472 + t22474 / 4.0_f64 - t22476 / 8.0_f64);
    (t22469, t22470, t22471, t22473, t22479)
}
