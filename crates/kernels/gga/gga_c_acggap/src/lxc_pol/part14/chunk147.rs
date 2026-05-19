//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 147/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk147<F: Float>(t127: F, t332: F, t335: F, t339: F, t363: F, t367: F, t374: F, t380: F, t392: F, t397: F, t409: F, t417: F, t418: F, t425: F, t431: F, t438: F) -> F {
    let t441 = -t332 - t335 * t339 / F::new(48.0) + t127 * t363 / F::new(96.0) - t367 * t374 / F::new(96.0) + t380 - t392 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t409 - t417 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t425 + F::cast_from(0.42874018118069736972e-3_f64) * t418 * t431 - F::cast_from(0.42874018118069736972e-3_f64) * t418 * t438;
    t441
}
