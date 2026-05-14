//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 996/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk996<F: Float>(t4680: F, t7575: F, t8609: F, t7564: F, t8613: F, t1181: F, t4718: F, t604: F, t7426: F, t31349: F, t3360: F, t4284: F, t2268: F, t30792: F, t7493: F, t7642: F, t8480: F) -> (F, F, F, F, F, F) {
    let t36276 = t7575 * t4680 * t8609;
    let t36279 = t7564 * t4680 * t8613;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36286 = t3360 * t31349 * t4284;
    let t36289 = t30792 * t2268;
    let t36292 = t7493 * t8480 * t7642;
    (t36276, t36279, t36283, t36286, t36289, t36292)
}
