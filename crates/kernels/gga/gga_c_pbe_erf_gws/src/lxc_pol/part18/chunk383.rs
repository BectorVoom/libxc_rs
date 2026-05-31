//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 383/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk383<F: Float>(t1193: F, t353: F, t338: F, t1174: F, t1181: F, t335: F) -> (F, F) {
    let t1194 = t353 * t1193;
    let t1195 = t338 * t1194;
    let t1198 = t1174 / F::cast_from(96.0_f64) + t1181 / F::cast_from(3072.0_f64) - t335 * t1195 / F::cast_from(96.0_f64);
    (t1195, t1198)
}
