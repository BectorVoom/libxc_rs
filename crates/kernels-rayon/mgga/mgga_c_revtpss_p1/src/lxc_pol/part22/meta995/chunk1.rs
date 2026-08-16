//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3383/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3383(t63262: f64, t63295: f64, t63334: f64, t63380: f64, t63473: f64, t63509: f64, t63540: f64, t63573: f64, t915: f64, t935: f64, t41578: f64, t6145: f64) -> (f64, f64) {
    let t63579 = 1.0_f64 * t915 * (t63262 + t63295 + t63334 + t63380 + t63473 + t63509 + t63540 + t63573) * t935;
    let t63581 = 0.16081979498692535067e2_f64 * t41578 * t6145;
    (t63579, t63581)
}
