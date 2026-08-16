//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2196/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2196(t1985: f64, t22934: f64, t26193: f64, t80722: f64, t80725: f64, t80728: f64, t80738: f64, t80744: f64, t90598: f64, t90602: f64, t90605: f64, t90609: f64, t90612: f64) -> f64 {
    let t90615 = t1985 * t26193 * t22934;
    let t90617 = 0.12793931631041761173e0_f64 * t80722;
    let t90621 = -0.16449340668482264365e-1_f64 * t90598 - 0.6579736267392905746e-1_f64 * t90602 - t90605 - 0.49348022005446793096e-1_f64 * t90609 + 0.3289868133696452873e-1_f64 * t90612 + 0.16449340668482264365e-1_f64 * t90615 + t90617 + 0.41123351671205660912e-2_f64 * t80725 - 0.11514538467937585055e0_f64 * t80728 - 0.41123351671205660912e-2_f64 * t80738 - t80744;
    t90621
}
