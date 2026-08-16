//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 76/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk76(t181: f64, t187: f64, t119: f64, t132: f64, t6: f64) -> (f64, f64, f64) {
    let t188 = t181 * t187;
    let t191 = 1.0_f64 + 0.65854491829355115987e0_f64 * t119 * t188;
    let t195 = t6 * t132;
    (t188, t191, t195)
}
