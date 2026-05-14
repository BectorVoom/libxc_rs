//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1079/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1079<F: Float>(t11217: F, t8510: F, t1492: F, t3640: F, t101: F, t11270: F, t190: F, t25047: F, t4049: F, t35415: F, t35419: F, t35422: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F) -> (F, F) {
    let t35451 = t8510 * t11217;
    let t35453 = t1492 * t3640;
    let t35455 = t11270 * t101;
    let t35458 = t35455 * t4049 * t190 * t25047;
    let t35460 = 0.59742541934307102628e-4 * t35415 + 0.5431140175846100239e-5 * t35419 - 0.27155700879230501195e-5 * t35422 + 0.23101203872956502753e-6 * t35429 - 0.5431140175846100239e-5 * t35432 - 0.5431140175846100239e-5 * t35435 - 0.3218855744218122075e-6 * t35439 - 0.84356546269123608434e-6 * t35443 - 0.84356546269123608434e-6 * t35447 + 0.22776267492663374277e-4 * t35449 - 0.24553279544970911497e-4 * t35451 + 0.3475929712541504153e-3 * t35453 + 0.7381197798548315738e-6 * t35458;
    (t35455, t35460)
}
