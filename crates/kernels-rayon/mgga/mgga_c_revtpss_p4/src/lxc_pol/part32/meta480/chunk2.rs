//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1721/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1721(t27383: f64, t27384: f64, t1583: f64, t605: f64, t30: f64, t4537: f64, t1468: f64, t775: f64, t890: f64, t33: f64, t892: f64, t4433: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27385 = t27383 * t27384;
    let t27387 = t605 * t1583;
    let t27391 = t30 * t4537;
    let t27395 = t1468 * t775;
    let t27402 = t1468 * t890;
    let t27763 = t892 * t33;
    let t27764 = t27763 * t4433;
    (t27385, t27387, t27391, t27395, t27402, t27763, t27764)
}
