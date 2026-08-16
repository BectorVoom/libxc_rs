//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1084/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1084<F: Float>(t2423: F, t804: F, t1332: F, t296: F, t6073: F, t6072: F, t6074: F, t793: F, t18471: F, t18474: F, t18477: F, t18479: F, t18512: F, t18514: F, t18518: F, t18521: F, t18523: F, t18527: F, t18529: F, t2074: F, t2424: F, t6838: F, t810: F, t8556: F) -> (F, F, F) {
    let t19477 = t804 * t2423;
    let t19482 = F::cast_from(0.47400060215270560269e1_f64) * t6073 * t1332 * t296;
    let t19487 = t793 * t6072 * t6074;
    let t19488 = F::cast_from(0.18960024086108224108e1_f64) * t19487;
    let t19492 = F::cast_from(18.0_f64) * t2074 * t2424 * t804 + F::cast_from(12.0_f64) * t6838 * t804 * t810 - F::cast_from(36.0_f64) * t19477 * t8556 - t18471 - t18474 + t18477 + t18479 + t18512 - t18514 + t18518 + t18521 - t18523 + t18527 + t18529 + t19482 - t19488;
    (t19482, t19488, t19492)
}
