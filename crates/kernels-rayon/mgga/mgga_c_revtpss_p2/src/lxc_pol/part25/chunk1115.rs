//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1115/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1115(t25255: f64, t807: f64, t2756: f64, t7038: f64, t2718: f64, t64: f64) -> (f64, f64, f64) {
    let t25256 = t807 * t25255;
    let t25257 = 0.11433071498151929859e-3_f64 * t25256;
    let t25258 = t7038 * t2756;
    let t25260 = t2718 * t64;
    (t25257, t25258, t25260)
}
