//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1299/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1299(t109: f64, t81438: f64, t81440: f64, t81443: f64, t81445: f64, t81447: f64, t81450: f64, t81452: f64, t510: f64, t652: f64, t1983: f64, t22584: f64, t22591: f64) -> (f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t81455 = piecewise3(t110, 0.0_f64, -t81438 - 11.0_f64 / 3.0_f64 * t81440 - 2.0_f64 * t81443 + t81445 - 3.0_f64 / 4.0_f64 * t81447 + 3.0_f64 / 4.0_f64 * t81450 - t81452 / 8.0_f64);
    let t81458 = 2.0_f64 * t652 * t510 * t81455;
    let t81469 = 9.0_f64 * t1983 * t22591 * t22584;
    (t81455, t81458, t81469)
}
