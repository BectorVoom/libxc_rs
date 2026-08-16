//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 200/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk200(t598: f64, t606: f64, t572: f64, t581: f64, t585: f64, t590: f64, t602: f64) -> f64 {
    let t607 = t598 * t606;
    let t609 = t572 / 96.0_f64 + t581 / 384.0_f64 - 0.38203125e-2_f64 * t585 + 0.42874018118069736972e-3_f64 * t590 + 0.10718504529517434243e-3_f64 * t602 - 0.15724046144802076034e-3_f64 * t607;
    t609
}
