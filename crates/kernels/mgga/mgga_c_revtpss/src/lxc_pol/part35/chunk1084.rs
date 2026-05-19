//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1084/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1084<F: Float>(t26310: F, t26312: F, t27924: F, t27937: F, t30035: F, t30037: F, t30039: F, t30041: F, t30043: F, t30045: F, t30246: F) -> F {
    let t30247 = -F::cast_from(0.85748036236139473944e-3_f64) * t30035 + F::cast_from(0.17149607247227894789e-1_f64) * t30037 + F::cast_from(0.22866142996303859718e-3_f64) * t27937 - F::cast_from(0.34299214494455789578e-2_f64) * t30039 - t26310 + t26312 - F::cast_from(0.85748036236139473944e-3_f64) * t30041 + t30043 / F::new(8.0) - F::cast_from(0.4065600224742826258e-3_f64) * t27924 + F::cast_from(0.68598428988911579156e-2_f64) * t30045 + t30246;
    t30247
}
