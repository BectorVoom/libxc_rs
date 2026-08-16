//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 988/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk988(t115432: f64, t22716: f64, t8631: f64, t114058: f64, t114061: f64, t114064: f64, t114073: f64, t114077: f64, t115420: f64, t115423: f64, t115428: f64, t115430: f64) -> f64 {
    let t115433 = 0.26044789391763585244e-1_f64 * t115432;
    let t115434 = t22716 * t8631;
    let t115435 = 0.63969658155208805863e-1_f64 * t115434;
    let t115436 = t114058 + t114061 - t114064 - 0.82246703342411321825e-2_f64 * t115420 + 0.82246703342411321824e-2_f64 * t115423 - 0.16449340668482264365e-1_f64 * t115428 - 0.38381794893125283518e-1_f64 * t115430 + t115433 + t115435 - t114073 - t114077;
    t115436
}
