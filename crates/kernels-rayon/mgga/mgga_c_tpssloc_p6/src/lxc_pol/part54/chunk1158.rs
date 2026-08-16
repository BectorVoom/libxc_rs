//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1158/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1158(t31394: f64, t829: f64, t235: f64, t31361: f64, t226: f64, t30675: f64, t30680: f64, t30683: f64, t30688: f64, t30692: f64, t31375: f64, t31379: f64, t31383: f64, t31387: f64, t31391: f64, t808: f64, t812: f64, t8560: f64) -> (f64, f64, f64) {
    let t31395 = t31394 * t829;
    let t31397 = t235 * t31361;
    let t31399 = -t30675 - t30680 - t30683 - t30688 + t30692 - t31375 - 0.16449340668482264365e-1_f64 * t31379 - t31383 - 0.82246703342411321825e-2_f64 * t31387 + 0.82246703342411321825e-2_f64 * t31391 + t808 * t8560 - t812 * t31395 + t226 * t31397;
    (t31395, t31397, t31399)
}
