//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3707/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3707(t17289: f64, t1803: f64, t1222: f64, t6652: f64, t697: f64, t42871: f64, t6628: f64) -> (f64, f64, f64) {
    let t70221 = t17289 * t1803;
    let t70225 = t1222 * t697 * t6652;
    let t70235 = t6628 * t42871;
    (t70221, t70225, t70235)
}
