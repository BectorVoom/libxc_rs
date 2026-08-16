//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 506/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk506(t2371: f64, t761: f64, t118: f64, t187: f64, t677: f64, t763: f64, t123: f64, t126: f64, t131: f64, t119: f64, t132: f64, t63: f64) -> (f64, f64, f64, f64, f64) {
    let t2373 = 0.11696447245269292414e1_f64 * t761 * t2371;
    let t2374 = t187 * t118;
    let t2375 = t677 * t763;
    let t2377 = 0.10843581300301739842e-1_f64 * t2374 * t2375;
    let t2385 = 1.0_f64 / t126 / t123 * t131;
    let t2386 = t132 * t119;
    let t2387 = t2386 * t63;
    (t2373, t2375, t2377, t2385, t2387)
}
