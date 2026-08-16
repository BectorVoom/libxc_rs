//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 156/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk156<F: Float>(t393: F, t395: F, t399: F, t401: F, t30: F) -> (F, F) {
    let t403 = -F::cast_from(0.632975e0_f64) * t393 - F::cast_from(0.29896666666666666667e0_f64) * t395 - F::cast_from(0.1023875e0_f64) * t399 - F::cast_from(0.82156666666666666667e-1_f64) * t401;
    let t404 = F::cast_from(1.0_f64) / t30;
    (t403, t404)
}
