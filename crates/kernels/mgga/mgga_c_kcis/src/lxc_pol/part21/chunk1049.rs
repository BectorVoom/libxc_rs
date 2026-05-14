//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1049/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1049<F: Float>(t2835: F, t3038: F, t3323: F, t3329: F, t10497: F, t1138: F, t10496: F, t364: F, t392: F, t11229: F, t1278: F, t3662: F, t3668: F, t3621: F, t11181: F, t413: F, t429: F) -> (F, F, F, F, F, F, F, F) {
    let t33827 = t3038 * t2835;
    let t33848 = t3323 * t3329;
    let t33853 = t1138 * t10497;
    let t33862 = t364 / t10496 / t392;
    let t34650 = t1278 * t11229;
    let t34662 = t3662 * t3668;
    let t34689 = t3621 * t3621;
    let t34690 = 1.0 / t34689;
    let t34814 = t413 / t11181 / t429;
    (t33827, t33848, t33853, t33862, t34650, t34662, t34690, t34814)
}
