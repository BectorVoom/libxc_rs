//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 947/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk947(t31361: f64, t814: f64, t2627: f64, t8543: f64, t23168: f64, t31378: f64, t2553: f64, t31376: f64, t6552: f64, t6637: f64, t22893: f64, t23164: f64, t31377: f64) -> (f64, f64, f64, f64, f64) {
    let t114649 = t814 * t31361;
    let t114655 = t2627 * t8543;
    let t114659 = t23168 * t31378;
    let t114663 = t6552 * t6637 * t31376 * t2553;
    let t114666 = t23164 * t22893 * t31377;
    (t114649, t114655, t114659, t114663, t114666)
}
