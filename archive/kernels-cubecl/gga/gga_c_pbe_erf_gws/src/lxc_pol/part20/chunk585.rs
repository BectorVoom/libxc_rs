//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 585/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk585<F: Float>(t3083: F, t833: F, t1164: F, t840: F, t1115: F, t2225: F, t2236: F, t2247: F, t2362: F, t2373: F, t2408: F, t2498: F, t2503: F, t3040: F, t3047: F, t3052: F, t3055: F, t3062: F, t3066: F, t3070: F, t3077: F, t3079: F, t827: F) -> (F, F, F) {
    let t3084 = t3083 * t833;
    let t3086 = t840 * t1164;
    let t3088 = t2498 * t833 / F::cast_from(96.0_f64) + t827 * t2503 / F::cast_from(96.0_f64) + t3040 * t833 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t2236 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2247 - t827 * t3047 / F::cast_from(96.0_f64) - t827 * t3052 / F::cast_from(48.0_f64) - t3055 * t2362 / F::cast_from(96.0_f64) - t1115 * t2373 / F::cast_from(48.0_f64) + t2408 * t3062 / F::cast_from(48.0_f64) + t3066 * t3070 / F::cast_from(48.0_f64) + t3077 * t3079 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t2225 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3084 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t3086;
    (t3084, t3086, t3088)
}
