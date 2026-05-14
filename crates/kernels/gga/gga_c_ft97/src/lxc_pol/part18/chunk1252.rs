//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1252/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1252<F: Float>(t1332: F, t7800: F, t47667: F, t5717: F, t101: F, t23249: F, t1882: F, t26225: F, t26375: F, t8392: F, t10974: F, t11392: F, t11397: F, t11437: F, t11472: F, t11473: F, t11490: F, t11501: F, t11556: F, t11557: F, t11593: F, t11604: F, t11867: F, t11906: F, t11982: F, t1339: F, t1901: F, t1922: F, t23299: F, t26435: F, t26441: F, t446: F, t452: F, t47659: F, t47666: F, t47809: F, t6454: F, t91739: F) -> (F,) {
    let t103423 = t1332 * t7800;
    let t103435 = t47667 * t5717;
    let t103439 = t101 * t23249;
    let t103453 = 2.0 / 9.0 * t1882 * t26225;
    let t103459 = 4.0 / 3.0 * t8392 * t26375;
    let t103471 = -2.0 / 9.0 * t1901 * t11472 * t26435 * t11982 - 4.0 / 9.0 * t1901 * t11556 * t103423 * t11437 - 8.0 / 9.0 * t11593 * t11472 * t26435 * t11604 + 4.0 / 27.0 * t1901 * t47809 * t26441 - 4.0 / 27.0 * t47666 * t103435 * t10974 - 8.0 / 27.0 * t47666 * t103439 * t11557 + 4.0 / 9.0 * t47659 * t103435 * t11397 + 8.0 / 9.0 * t47659 * t103439 * t11473 + 4.0 / 9.0 * t47659 * t91739 * t11867 - t103453 - 2.0 / 3.0 * t1901 * t11490 * t23249 * t11501 + t103459 - t446 * t452 * t1922 * t6454 / 3.0 - t446 * t452 * t1339 * t11392 / 3.0 - 2.0 / 9.0 * t1901 * t11906 * t23299;
    (t103471,)
}
