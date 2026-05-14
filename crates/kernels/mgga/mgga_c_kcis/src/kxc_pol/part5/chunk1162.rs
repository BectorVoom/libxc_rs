//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1162/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1162<F: Float>(t12159: F, t7080: F, t1380: F, t613: F, t5726: F, t5732: F, t3970: F, t7072: F, t1368: F, t5691: F, t5698: F, t1650: F, t1938: F, t833: F, t16963: F, t16831: F, t531: F) -> (F, F, F, F, F, F, F, F) {
    let t21468 = t12159 * t7080;
    let t21469 = t21468 * t1380;
    let t21470 = t613 * t21469;
    let t21473 = t5726 * t5732;
    let t21474 = t613 * t21473;
    let t21477 = t3970 * t7072;
    let t21478 = t1368 * t21477;
    let t21480 = t5691 * t5698;
    let t21484 = t1650 * t1938;
    let t21485 = t21484 * t833;
    let t21486 = t16963 * t21485;
    let t21489 = t16831 * t531;
    (t21470, t21474, t21478, t21480, t21484, t21485, t21486, t21489)
}
