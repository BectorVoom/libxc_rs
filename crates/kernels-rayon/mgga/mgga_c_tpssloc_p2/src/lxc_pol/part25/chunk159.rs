//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 159/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk159(t466: f64, t491: f64, t477: f64, t68: f64, t470: f64, t254: f64, t193: f64, t336: f64, t425: f64, t453: f64, t455: f64, t265: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t492 = t466 * t491;
    let t493 = t68 * t477;
    let t494 = t493 * t491;
    let t496 = t470 * t494 + 1.0_f64;
    let t497 = 1.0_f64 / t496;
    let t498 = t254 * t497;
    let t500 = t492 * t498 + 1.0_f64;
    let t501 = f64::ln(t500);
    let t504 = t193 * t336 * t501 - t425 + t453 + t455;
    let t505 = t265 < t504;
    (t492, t493, t494, t496, t498, t500, t504)
}
