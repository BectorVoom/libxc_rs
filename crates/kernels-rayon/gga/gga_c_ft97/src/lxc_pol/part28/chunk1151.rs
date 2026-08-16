//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1151/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1151(t1017: f64, t139418: f64, t28: f64, t89: f64, t26768: f64, t5778: f64, t32967: f64, t3408: f64, t34939: f64, t376: f64, t34931: f64, t147730: f64, t39749: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148593 = t89 * t28 * t139418 * t1017;
    let t148597 = t89 * t28 * t5778 * t26768;
    let t148601 = t89 * t28 * t32967 * t3408;
    let t148604 = t89 * t376 * t34939;
    let t148607 = t89 * t376 * t34931;
    let t148611 = t446 * t39749 * t147730;
    (t148593, t148597, t148601, t148604, t148607, t148611)
}
