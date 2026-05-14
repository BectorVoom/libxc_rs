//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1239/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1239<F: Float>(t33724: F, t33730: F, t43414: F, t44193: F, t44198: F, t52389: F, t52391: F, t52393: F, t58348: F, t58352: F, t58356: F, t58360: F, t58363: F, t58367: F, t26496: F, t26497: F, t52395: F, t58375: F, t58378: F, t58381: F, t58384: F, t58388: F, t58392: F, t58397: F, t58401: F, t58405: F, t58409: F, t58412: F) -> (F, F) {
    let t58701 = -0.18396666666666666667e0 * t44193 + 0.11038e1 * t44198 - 0.53675555555555555556e0 * t43414 + 0.12524296296296296297e1 * t33724 + 0.98115555555555555556e0 * t33730 + 0.24154e1 * t58348 + 0.99342e0 * t58352 - 0.22076e0 * t58356 - 0.298026e1 * t58360 + 0.66228e0 * t58363 - 0.11038e0 * t58367 + 0.40256666666666666668e0 * t52389 + 0.24154e1 * t52391 + 0.44729629629629629629e0 * t52393;
    let t58714 = -0.16102666666666666667e1 * t52395 + t26496 + t26497 - 0.20128333333333333334e1 * t58375 + 0.72462e1 * t58378 - 0.80513333333333333332e0 * t58381 - 0.108693e2 * t58384 + 0.44152e0 * t58388 + 0.49671e0 * t58392 + 0.40256666666666666666e1 * t58397 - 0.72462e1 * t58401 - 0.60384999999999999999e0 * t58405 + 0.72462e1 * t58409 + 0.181155e1 * t58412;
    (t58701, t58714)
}
