//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 226/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk226(t340: f64, t338: f64) -> (f64, f64, f64) {
    let t992 = t340 * t340;
    let t993 = 1.0_f64 / t992;
    let t994 = t338 * t993;
    (t992, t993, t994)
}
