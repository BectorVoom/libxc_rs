//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1282/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1282(t2717: f64, t7106: f64, t31334: f64, t6579: f64, t23185: f64, t31333: f64, t82074: f64, t31316: f64, t6547: f64, t31361: f64, t814: f64, t23168: f64, t31378: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114601 = t2717 * t7106;
    let t114606 = t6579 * t31334;
    let t114613 = t23185 * t82074 * t31333;
    let t114615 = t6547 * t31316;
    let t114649 = t814 * t31361;
    let t114659 = t23168 * t31378;
    (t114601, t114606, t114613, t114615, t114649, t114659)
}
