//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 877/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk877(t121: f64, t131: f64, t141: f64, t22: f64, t2456: f64, t624: f64) -> (f64, f64, f64) {
    let t9282 = 1.0_f64 / t131 / t141 * t121 / 4.0_f64;
    let t9283 = t9282 * t22;
    let t9285 = t2456 * t624;
    (t9282, t9283, t9285)
}
