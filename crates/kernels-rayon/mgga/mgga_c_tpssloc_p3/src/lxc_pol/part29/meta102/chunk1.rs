//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 661/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk661(t109: f64, t2358: f64, t656: f64, t2327: f64, t2328: f64, t2333: f64, t64: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t2359 = t656 * t2358;
    let t2363 = piecewise3(t110, 0.0_f64, t2327 + 2.0_f64 / 3.0_f64 * t2328 + t64 * t2333 / 4.0_f64 - t64 * t2359 / 8.0_f64);
    (t2359, t2363)
}
