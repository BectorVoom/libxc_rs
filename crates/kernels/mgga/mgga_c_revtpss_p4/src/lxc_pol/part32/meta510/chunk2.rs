//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1803/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1803<F: Float>(t26450: F, t26454: F, t26457: F, t27240: F, t27246: F, t27251: F, t27254: F, t29616: F, t29618: F, t29620: F, t30378: F) -> F {
    let t30379 = t26450 - t26454 + t26457 + F::cast_from(0.22866142996303859718e-3_f64) * t27240 + F::cast_from(0.17149607247227894789e-2_f64) * t29616 + F::cast_from(0.68598428988911579156e-2_f64) * t29618 - F::cast_from(0.85748036236139473944e-3_f64) * t29620 - F::cast_from(0.4065600224742826258e-3_f64) * t27251 + F::cast_from(0.57165357490759649296e-4_f64) * t27254 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t27246 + t30378;
    t30379
}
