//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1197/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1197<F: Float>(t108587: F, t108590: F, t108592: F, t108601: F, t114564: F, t114566: F, t96323: F, t96326: F, t96341: F, t96342: F, t98218: F, t98220: F, t98224: F, t98260: F) -> F {
    let t115052 = -t96323 - F::cast_from(0.3658582879408617555e-2_f64) * t98218 + F::cast_from(0.34299214494455789577e-3_f64) * t108587 - F::cast_from(0.54214778996945588151e-4_f64) * t98220 - F::cast_from(0.24009450146119052704e-1_f64) * t108590 + F::cast_from(0.12004725073059526352e-1_f64) * t108592 - F::cast_from(0.68026775414003982662e-1_f64) * t98224 + t96326 - F::cast_from(0.85748036236139473944e-3_f64) * t114564 + F::cast_from(0.51448821741683684367e-2_f64) * t114566 - F::new(35.0) / F::new(36.0) * t98260 - t96341 + t96342 + F::cast_from(0.85748036236139473944e-4_f64) * t108601;
    t115052
}
