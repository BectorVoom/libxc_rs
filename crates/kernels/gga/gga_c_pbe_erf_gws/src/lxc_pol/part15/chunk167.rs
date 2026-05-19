//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 167/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk167<F: Float>(t449: F, t75: F, t393: F, t395: F, t399: F, t401: F) -> (F, F) {
    let t450 = t75 * t449;
    let t455 = -F::cast_from(0.86308333333333333334e0_f64) * t393 - F::new(0.301925e0) * t395 - F::new(0.5501625e-1) * t399 - F::new(0.82785e-1) * t401;
    (t450, t455)
}
