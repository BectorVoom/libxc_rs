//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1206/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1206<F: Float>(t26314: F, t26319: F, t26339: F, t26343: F, t26363: F, t26365: F, t26367: F, t26369: F, t26372: F, t26376: F, t26379: F, t26382: F, t26385: F, t26388: F, t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26324: F, t26394: F, t26396: F, t26398: F, t26406: F, t26409: F, t26412: F, t26415: F) -> (F, F) {
    let t26523 = -0.89459259259259259259e0 * t26339 - 0.301925e0 * t26343 - 0.5519e0 * t26363 - 0.18396666666666666667e0 * t26365 + 0.22076e0 * t26367 + 0.98115555555555555555e-1 * t26369 + 0.40256666666666666668e0 * t26314 + 0.11038e1 * t26372 - 0.8585111111111111111e-1 * t26376 - 0.82785e-1 * t26379 - 0.22076e0 * t26382 + 0.66228e0 * t26385 - 0.11038e0 * t26388 + 0.24154e1 * t26319;
    let t26539 = -0.80513333333333333332e0 * t26324 - 0.132456e1 * t26394 + 0.22076e0 * t26396 - 0.3883875e1 * t26398 + 0.24154e1 * t26280 - 0.72462e1 * t26284 - 0.60384999999999999999e0 * t26293 + 0.72462e1 * t26296 + 0.181155e1 * t26304 - 0.16102666666666666667e1 * t26311 + 0.132456e1 * t26406 - 0.99342e0 * t26409 - 0.82785e-1 * t26412 + 0.198684e1 * t26415;
    (t26523, t26539)
}
