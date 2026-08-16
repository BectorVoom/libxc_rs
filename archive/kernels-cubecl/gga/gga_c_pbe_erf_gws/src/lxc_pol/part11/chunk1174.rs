//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1174/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1174<F: Float>(t16441: F, t16444: F, t16446: F, t16454: F, t16467: F, t16471: F, t16480: F, t18021: F, t26386: F, t26399: F, t26402: F, t26404: F, t26411: F, t26415: F, t30127: F, t30131: F, t33381: F, t33385: F, t33389: F, t42244: F) -> F {
    let t48609 = -F::cast_from(0.23704668394691377059e-1_f64) * t30127 - F::cast_from(0.29725654166942986832e-2_f64) * t30131 + t16441 + t16444 - t16446 - t16454 - t16467 - t16471 + t16480 + F::cast_from(0.13871971944573393855e-1_f64) * t26386 - F::cast_from(0.23704668394691377059e-1_f64) * t26399 - F::cast_from(0.59451308333885973663e-2_f64) * t26402 - F::cast_from(0.1035981803916141664e0_f64) * t26404 - F::cast_from(0.37806488667769341401e0_f64) * t33381 - F::cast_from(0.14369080812305530913e0_f64) * t33385 + F::cast_from(0.39507780657818961764e-1_f64) * t33389 - t18021 + F::cast_from(0.79015561315637923528e-1_f64) * t26411 - F::cast_from(0.79015561315637923528e-2_f64) * t42244 - F::cast_from(0.75612977335538682804e0_f64) * t26415;
    t48609
}
