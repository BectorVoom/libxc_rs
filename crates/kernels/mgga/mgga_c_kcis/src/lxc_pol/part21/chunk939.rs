//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 939/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk939<F: Float>(t2539: F, t26398: F, t7612: F, t8522: F, t235: F, t3703: F, t2169: F, t2801: F, t441: F, t2533: F, t7630: F, t2161: F, t2770: F, t2153: F, t2626: F, t2538: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26399 = t26398 * t2539;
    let t26400 = 2.0 * t26399;
    let t26401 = t8522 * t7612;
    let t26402 = 4.0 * t26401;
    let t26403 = t235 * t3703;
    let t26404 = t2169 * t26403;
    let t26405 = t26404 / 16.0;
    let t26406 = t2801 * t441;
    let t26407 = t2169 * t26406;
    let t26408 = t26407 / 16.0;
    let t26409 = t2533 * t7630;
    let t26410 = 2.0 * t26409;
    let t26411 = t2161 * t2770;
    let t26416 = t2153 * t2626;
    let t26417 = t2538 * t26416;
    (t26399, t26400, t26401, t26402, t26405, t26408, t26409, t26410, t26411, t26416, t26417)
}
