//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 731/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk731<F: Float>(t2409: F, t3067: F, t4216: F, t4104: F, t4108: F, t4169: F, t4172: F, t4174: F, t4176: F, t4178: F, t4180: F) -> (F, F) {
    let t4218 = t2409 * t3067 * t4216;
    let t4227 = t4169 / F::cast_from(48.0_f64) - t4172 / F::cast_from(48.0_f64) - t4104 - t4174 / F::cast_from(24.0_f64) + t4176 / F::cast_from(384.0_f64) - t4178 / F::cast_from(384.0_f64) - t4108 - t4180 / F::cast_from(192.0_f64);
    (t4218, t4227)
}
