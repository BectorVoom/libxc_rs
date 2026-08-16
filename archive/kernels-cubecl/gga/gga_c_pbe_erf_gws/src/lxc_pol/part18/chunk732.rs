//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 732/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk732<F: Float>(t1154: F, t4043: F, t1158: F, t4049: F, t4035: F, t4047: F, t4169: F, t4172: F, t4174: F, t4176: F) -> F {
    let t4178 = t4043 * t1154;
    let t4180 = t4049 * t1158;
    let t4182 = t4169 / F::cast_from(96.0_f64) - t4172 / F::cast_from(96.0_f64) - t4035 - t4174 / F::cast_from(48.0_f64) + t4176 / F::cast_from(768.0_f64) - t4178 / F::cast_from(768.0_f64) - t4047 - t4180 / F::cast_from(384.0_f64);
    t4182
}
