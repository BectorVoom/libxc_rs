//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1153/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1153(t147730: f64, t5899: f64, t95344: f64, t34943: f64, t375: f64, t89: f64, t147590: f64, t27: f64, t526: f64, t139453: f64, t139485: f64, t139493: f64, t139496: f64, t148593: f64, t148597: f64, t148601: f64, t148604: f64, t148607: f64, t148611: f64, t148616: f64, t148621: f64, t148625: f64) -> (f64, f64, f64, f64) {
    let t148629 = t5899 * t95344 * t147730;
    let t148632 = t89 * t375 * t34943;
    let t148636 = t89 * t27 * t526 * t147590;
    let t148638 = 2.0_f64 / 3.0_f64 * t148593 + 4.0_f64 / 3.0_f64 * t148597 - 2.0_f64 * t148601 - 2.0_f64 / 9.0_f64 * t148604 - 4.0_f64 / 9.0_f64 * t148607 + t139453 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t148611 + t148616 / 3.0_f64 + t148621 / 12.0_f64 - 4.0_f64 * t148625 + t139485 / 27.0_f64 + t148629 / 3.0_f64 + t148632 / 9.0_f64 - t148636 / 3.0_f64 - t139493 + t139496;
    (t148629, t148632, t148636, t148638)
}
