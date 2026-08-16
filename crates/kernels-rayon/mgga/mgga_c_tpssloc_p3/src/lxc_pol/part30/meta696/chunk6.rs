//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2233/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2233(t13397: f64, t16816: f64, t25261: f64, t4182: f64, t4234: f64, t4281: f64, t4291: f64, t81633: f64, t829: f64, t87536: f64, t87545: f64, t87547: f64, t87566: f64, t87582: f64, t87584: f64, t87602: f64, t98494: f64, t98541: f64, t98546: f64, t98549: f64, t98553: f64, t98564: f64) -> f64 {
    let t98566 = t87536 - t87545 - t87547 - 0.12793931631041761173e0_f64 * t81633 - t87566 - 2.0_f64 * t4291 * t25261 * t4234 - t4291 * t98541 * t829 - 0.16449340668482264365e-1_f64 * t98546 + 0.82246703342411321825e-2_f64 * t98549 - 0.82246703342411321825e-2_f64 * t98553 + 2.0_f64 * t4281 * t98494 * t4182 + t87582 - t87584 + 6.0_f64 * t4281 * t98541 * t4182 - 6.0_f64 * t13397 * t98541 * t16816 + 0.38381794893125283518e-1_f64 * t98564 + t87602;
    t98566
}
