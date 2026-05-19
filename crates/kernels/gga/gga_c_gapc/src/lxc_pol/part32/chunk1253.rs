//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1253/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1253<F: Float>(t190: F, t25047: F, t35455: F, t4049: F, t35415: F, t35419: F, t35422: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F, t35451: F, t35453: F) -> F {
    let t35458 = t35455 * t4049 * t190 * t25047;
    let t35460 = F::cast_from(0.59742541934307102628e-4_f64) * t35415 + F::cast_from(0.5431140175846100239e-5_f64) * t35419 - F::cast_from(0.27155700879230501195e-5_f64) * t35422 + F::cast_from(0.23101203872956502753e-6_f64) * t35429 - F::cast_from(0.5431140175846100239e-5_f64) * t35432 - F::cast_from(0.5431140175846100239e-5_f64) * t35435 - F::cast_from(0.3218855744218122075e-6_f64) * t35439 - F::cast_from(0.84356546269123608434e-6_f64) * t35443 - F::cast_from(0.84356546269123608434e-6_f64) * t35447 + F::cast_from(0.22776267492663374277e-4_f64) * t35449 - F::cast_from(0.24553279544970911497e-4_f64) * t35451 + F::cast_from(0.3475929712541504153e-3_f64) * t35453 + F::cast_from(0.7381197798548315738e-6_f64) * t35458;
    t35460
}
