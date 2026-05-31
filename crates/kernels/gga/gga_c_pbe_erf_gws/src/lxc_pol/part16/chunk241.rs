//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 241/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk241<F: Float>(t108: F, t418: F, t422: F, t726: F, t728: F, t266: F, t9: F) -> (F, F) {
    let t732 = (F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t418 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t422) * t108;
    let t735 = t266 * t9;
    (t732, t735)
}
