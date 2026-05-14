//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1005/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1005<F: Float>(t11120: F, t37452: F, t4441: F, t1630: F, t4466: F, t7913: F, t7905: F, t533: F, t1593: F, t3099: F, t35: F, t929: F, t938: F, t15631: F, t37482: F, t4545: F, t463: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t58268 = t37452 * t4441 * t11120;
    let t58286 = t1630 * t4466;
    let t58293 = t7913 * t4441;
    let t58341 = t7905 * t4441;
    let t58348 = t533 * t4441;
    let t58513 = t1593 * t4466;
    let t58559 = t35 * t3099;
    let t58580 = t929 * t938;
    let t58935 = t37482 * t15631;
    let t59506 = t463 * t4545;
    (t58268, t58286, t58293, t58341, t58348, t58513, t58559, t58580, t58935, t59506)
}
