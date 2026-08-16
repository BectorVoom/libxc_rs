//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 331/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk331<F: Float>(t138: F, t514: F, t981: F, t985: F, t101: F, t417: F, t533: F) -> (F, F, F) {
    let t987 = t138 * t981 - t514 * t985;
    let t988 = t101 * t987;
    let t991 = F::cast_from(2.0_f64) * t417 + F::cast_from(2.0_f64) * t533;
    (t987, t988, t991)
}
