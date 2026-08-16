//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1160/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1160(t343: f64, t613: f64, t136: f64, t1007: f64, t1968: f64, t1967: f64, t800: f64) -> (f64, f64, f64, f64) {
    let t7105 = t613 * t343;
    let t7106 = t7105 * t136;
    let t7110 = t1968 * t1007 / 288.0_f64;
    let t7111 = t1967 * t800;
    (t7105, t7106, t7110, t7111)
}
