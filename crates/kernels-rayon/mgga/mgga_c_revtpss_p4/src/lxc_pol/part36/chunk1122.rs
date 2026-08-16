//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1122/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1122(t2718: f64, t867: f64, t1950: f64, t2453: f64, t2458: f64, t25372: f64, t25410: f64) -> (f64, f64, f64, f64) {
    let t25416 = t867 * t2718;
    let t25422 = t2453 * t1950;
    let t25424 = 0.11565819519348392139e-2_f64 * t25422 * t2458;
    let t25431 = t25372 * t25410;
    (t25416, t25422, t25424, t25431)
}
