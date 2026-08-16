//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 134/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk134(t475: f64, t46: f64, t47: f64, rho1: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t476 = t475 - 1.0_f64;
    let t477 = 1.0_f64 / t476;
    let t478 = sigma2 * sigma2;
    let t479 = t477 * t478;
    let t480 = t46 * t46;
    let t481 = t480 * rho1;
    let t483 = 1.0_f64 / t47 / t481;
    (t476, t477, t478, t479, t480, t483)
}
