//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1232/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1232<F: Float>(t7716: F, t7725: F, t5984: F, t7713: F, t2064: F, t2899: F, t2902: F, t2029: F, t7575: F, t154: F, t2048: F, t276: F, t7350: F) -> (F, F, F, F, F) {
    let t21494 = t7725 * t7716;
    let t21496 = t5984 * t7713;
    let t21499 = t2899 * t2064 * t2902;
    let t21500 = F::cast_from(0.28582678745379824648e-3_f64) * t21499;
    let t21518 = t7575 * t2029;
    let t21527 = t276 * t154 * t2048 * t7350;
    (t21494, t21496, t21500, t21518, t21527)
}
