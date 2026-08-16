//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 906/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk906(t11520: f64, t11588: f64, t300: f64, t2979: f64, t983: f64, t11291: f64, t11293: f64, t11296: f64, t11303: f64, t11382: f64, t11390: f64, t11392: f64, t11394: f64, t11398: f64) -> (f64, f64, f64) {
    let t11590 = t300 * (t11520 + t11588);
    let t11591 = t300 * t2979;
    let t11593 = 0.17544670867903938621e1_f64 * t11591 * t983;
    let t11594 = t11291 + t11293 + t11296 - t11303 + t11382 + t11390 - t11392 - t11394 - t11398 + t11590 - t11593;
    (t11590, t11593, t11594)
}
