//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 394/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk394<F: Float>(t1326: F, t60: F, t40: F, t409: F, t461: F, t37: F, t38: F, t36: F, t88: F, t35: F, t39: F) -> (F, F, F, F, F, F, F, F) {
    let t1327 = t60 * t1326;
    let t1328 = t40 * t1327;
    let t1329 = t409 * t461;
    let t1330 = F::cast_from(8.0_f64) * t1329;
    let t1331 = t38 * t37;
    let t1332 = F::cast_from(1.0_f64) / t1331;
    let t1333 = t36 * t1332;
    let t1334 = t1333 * t88;
    let t1335 = F::cast_from(20.0_f64) * t1334;
    let t1336 = t35 * t39;
    (t1327, t1328, t1330, t1331, t1332, t1333, t1335, t1336)
}
