//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 155/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk155(t407: f64, t456: f64, t182: f64, t441: f64, t119: f64, t151: f64, t451: f64, t455: f64) -> (f64, f64, f64) {
    let t457 = t456 * t407;
    let t460 = t182 * t441;
    let t463 = t451 - t455 - 0.65854491829355115987e0_f64 * t151 * t457 + 0.65854491829355115987e0_f64 * t119 * t460;
    (t457, t460, t463)
}
