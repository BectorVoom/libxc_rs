//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 475/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk475<F: Float>(t2200: F, t333: F, t338: F, t348: F, t837: F, t855: F, t863: F) -> (F, F, F) {
    let t2201 = t2200 * t333;
    let t2204 = F::new(35.0) / F::new(432.0) * t348 * t2201 * t338;
    let t2206 = t863 * t855 * t837;
    (t2201, t2204, t2206)
}
