//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 646/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk646(t43: f64, t48: f64, t624: f64, t49: f64, t606: f64, t613: f64) -> (f64, f64, f64) {
    let t6968 = t43 * t48;
    let t6971 = 8.0_f64 / 3.0_f64 * t624;
    let t6972 = -8.0_f64 / 3.0_f64 * t613 * t49 + 5.0_f64 / 6.0_f64 * t6968 * t606 + t6971;
    (t6968, t6971, t6972)
}
