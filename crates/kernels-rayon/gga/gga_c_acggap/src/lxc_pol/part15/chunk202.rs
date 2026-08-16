//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 202/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk202(t572: f64, t581: f64, t585: f64, t590: f64, t602: f64, t607: f64) -> f64 {
    let t633 = t572 / 48.0_f64 + t581 / 192.0_f64 - 0.7640625e-2_f64 * t585 + 0.85748036236139473944e-3_f64 * t590 + 0.21437009059034868486e-3_f64 * t602 - 0.31448092289604152069e-3_f64 * t607;
    t633
}
