//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1260/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1260<F: Float>(t5122: F, t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t52389: F, t52391: F, t52393: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F, t26599: F, t26600: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F) -> (F, F, F) {
    let t59263 = t5122 * t5122;
    let t59281 = -0.23154444444444444445e0 * t44193 + 0.13892666666666666667e1 * t44198 - 0.91817777777777777776e0 * t43414 + 0.21424148148148148148e1 * t33724 + 0.12349037037037037037e1 * t33730 + 0.41318e1 * t58348 + 0.125034e1 * t58352 - 0.27785333333333333334e0 * t58356 - 0.375102e1 * t58360 + 0.83356e0 * t58363 - 0.13892666666666666667e0 * t58367 + 0.68863333333333333332e0 * t52389 + 0.41318e1 * t52391 + 0.76514814814814814814e0 * t52393;
    let t59294 = -0.27545333333333333332e1 * t52395 + t26599 + t26600 - 0.34431666666666666667e1 * t58375 + 0.123954e2 * t58378 - 0.13772666666666666667e1 * t58381 - 0.185931e2 * t58384 + 0.55570666666666666666e0 * t58388 + 0.62517e0 * t58392 + 0.68863333333333333334e1 * t58397 - 0.123954e2 * t58401 - 0.103295e1 * t58405 + 0.123954e2 * t58409 + 0.309885e1 * t58412;
    (t59263, t59281, t59294)
}
