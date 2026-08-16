//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1174/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1174(t16441: f64, t16444: f64, t16446: f64, t16454: f64, t16467: f64, t16471: f64, t16480: f64, t18021: f64, t26386: f64, t26399: f64, t26402: f64, t26404: f64, t26411: f64, t26415: f64, t30127: f64, t30131: f64, t33381: f64, t33385: f64, t33389: f64, t42244: f64) -> f64 {
    let t48609 = -0.23704668394691377059e-1_f64 * t30127 - 0.29725654166942986832e-2_f64 * t30131 + t16441 + t16444 - t16446 - t16454 - t16467 - t16471 + t16480 + 0.13871971944573393855e-1_f64 * t26386 - 0.23704668394691377059e-1_f64 * t26399 - 0.59451308333885973663e-2_f64 * t26402 - 0.1035981803916141664e0_f64 * t26404 - 0.37806488667769341401e0_f64 * t33381 - 0.14369080812305530913e0_f64 * t33385 + 0.39507780657818961764e-1_f64 * t33389 - t18021 + 0.79015561315637923528e-1_f64 * t26411 - 0.79015561315637923528e-2_f64 * t42244 - 0.75612977335538682804e0_f64 * t26415;
    t48609
}
