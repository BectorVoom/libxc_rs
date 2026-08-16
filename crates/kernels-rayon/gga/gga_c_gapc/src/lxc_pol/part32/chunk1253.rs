//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1253/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1253(t190: f64, t25047: f64, t35455: f64, t4049: f64, t35415: f64, t35419: f64, t35422: f64, t35429: f64, t35432: f64, t35435: f64, t35439: f64, t35443: f64, t35447: f64, t35449: f64, t35451: f64, t35453: f64) -> f64 {
    let t35458 = t35455 * t4049 * t190 * t25047;
    let t35460 = 0.59742541934307102628e-4_f64 * t35415 + 0.5431140175846100239e-5_f64 * t35419 - 0.27155700879230501195e-5_f64 * t35422 + 0.23101203872956502753e-6_f64 * t35429 - 0.5431140175846100239e-5_f64 * t35432 - 0.5431140175846100239e-5_f64 * t35435 - 0.3218855744218122075e-6_f64 * t35439 - 0.84356546269123608434e-6_f64 * t35443 - 0.84356546269123608434e-6_f64 * t35447 + 0.22776267492663374277e-4_f64 * t35449 - 0.24553279544970911497e-4_f64 * t35451 + 0.3475929712541504153e-3_f64 * t35453 + 0.7381197798548315738e-6_f64 * t35458;
    t35460
}
