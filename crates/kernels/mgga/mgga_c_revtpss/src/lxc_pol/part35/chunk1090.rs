//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1090/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1090<F: Float>(t26450: F, t26454: F, t26457: F, t27240: F, t27246: F, t27251: F, t27254: F, t29616: F, t29618: F, t29620: F, t30378: F) -> F {
    let t30379 = t26450 - t26454 + t26457 + F::new(0.22866142996303859718e-3) * t27240 + F::new(0.17149607247227894789e-2) * t29616 + F::new(0.68598428988911579156e-2) * t29618 - F::new(0.85748036236139473944e-3) * t29620 - F::new(0.4065600224742826258e-3) * t27251 + F::new(0.57165357490759649296e-4) * t27254 + F::new(7.0) / F::new(36.0) * t27246 + t30378;
    t30379
}
