//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 394/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk394(t1326: f64, t60: f64, t40: f64, t409: f64, t461: f64, t37: f64, t38: f64, t36: f64, t88: f64, t35: f64, t39: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1327 = t60 * t1326;
    let t1328 = t40 * t1327;
    let t1329 = t409 * t461;
    let t1330 = 8.0_f64 * t1329;
    let t1331 = t38 * t37;
    let t1332 = 1.0_f64 / t1331;
    let t1333 = t36 * t1332;
    let t1334 = t1333 * t88;
    let t1335 = 20.0_f64 * t1334;
    let t1336 = t35 * t39;
    (t1327, t1328, t1330, t1331, t1332, t1333, t1335, t1336)
}
