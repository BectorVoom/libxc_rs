//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 435/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk435<F: Float>(t2007: F, t929: F, t120: F, t383: F, t3056: F, t528: F, t72: F, t1005: F, t126: F, t1631: F, t2014: F, t2021: F, t534: F, t139: F, t527: F, t1013: F, t549: F) -> (F, F, F, F, F, F, F) {
    let t3359 = t2007 * t929;
    let t3360 = t120 * t383;
    let t3363 = t528 * t3056;
    let t3364 = t3363 * t120;
    let t3366 = t929 * t383;
    let t3368 = t72 * t3366 * t120;
    let t3371 = t1005 * t383;
    let t3374 = t3056 * t126;
    let t3379 = -0.11705142615505742e0 * t3359 * t3360 + 0.23410285231011484e0 * t3364 - 0.26564305359272358183e-2 * t2014 * t3368 + 0.319782988780431561e-1 * t2021 * t3371 - 0.532971647967385935e-1 * t534 * t3374 + 0.13977476158628290272e-1 * t1631 * t3371;
    let t3380 = t139 * t3379;
    let t3381 = t527 * t3380;
    let t3383 = t549 * t1013;
    (t3359, t3360, t3363, t3379, t3380, t3381, t3383)
}
